//! DMA
//!
//! Some of this implementation was scraped from:
//! https://github.com/stm32-rs/stm32f7xx-hal

// TODO
// - move chan_num field a typenum type state
// - finish the impl to allow peripheral/mem transfers, currently focused on
//   mem-to-mem only
// - add src/dst cached support once caches are enabled
// - consider using https://crates.io/crates/embedded-dma

use crate::ccu::Ccu;
use crate::pac::{
    ccu::{BusClockGating0, BusSoftReset0},
    dma::channel::ChannelEnable,
    dma::{AutoGating, Security, Status, DMA},
};
use as_slice::AsSlice;
use core::{
    mem,
    ops::Deref,
    ops::DerefMut,
    pin::Pin,
    sync::atomic::{self, Ordering},
};
use cortex_a::asm;

pub mod descriptor;
pub use descriptor::Descriptor;
use descriptor::{AddressMode, BurstLength, Config, DataWidth, DrqPort, Param};

pub trait DmaExt {
    type Parts;

    fn split(self, ccu: &mut Ccu) -> Self::Parts;
}

pub struct Dma {
    pub ch0: Channel,
    // TODO ch1..=ch7
}

pub struct Channel {
    dma: DMA,
    chan_num: ChannelNumber,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ChannelNumber {
    Ch0,
    // Ch1..=Ch7
}

impl DmaExt for DMA {
    type Parts = Dma;

    fn split(self, ccu: &mut Ccu) -> Self::Parts {
        ccu.bsr0.rstr().modify(BusSoftReset0::Dma::Clear);
        ccu.bsr0.rstr().modify(BusSoftReset0::Dma::Set);
        ccu.bcg0.enr().modify(BusClockGating0::Dma::Set);

        let mut dma = unsafe { DMA::from_paddr() };

        dma.auto_gating.modify(AutoGating::MasterClock::Enable);
        dma.auto_gating.modify(AutoGating::Channel::Enable);
        dma.auto_gating.modify(AutoGating::Common::Enable);

        Dma {
            ch0: Channel {
                dma,
                chan_num: ChannelNumber::Ch0,
            },
        }
    }
}

#[derive(Debug)]
pub struct TransferResources<SrcBuf, DstBuf> {
    // TODO - desc could be a slice, to-be-chained/linked
    desc: Pin<&'static mut Descriptor>,
    src_buffer: Pin<SrcBuf>,
    dst_buffer: Pin<DstBuf>,
}

impl<SrcBuf, DstBuf> TransferResources<SrcBuf, DstBuf>
where
    SrcBuf: 'static,
    DstBuf: 'static,
{
    pub fn mem_to_mem<SrcWord, DstWord>(
        mut desc: Pin<&'static mut Descriptor>,
        src_buffer: Pin<SrcBuf>,
        dst_buffer: Pin<DstBuf>,
    ) -> Self
    where
        SrcBuf: Deref,
        SrcBuf::Target: Buffer<SrcWord>,
        DstBuf: DerefMut,
        DstBuf::Target: Buffer<DstWord>,
        SrcWord: SupportedWordSize,
        DstWord: SupportedWordSize,
    {
        // TODO
        // - check descriptor for half-word alignment
        // - check descriptor address
        assert!(src_buffer.size() <= u32::max_value() as usize);
        assert!(dst_buffer.size() <= u32::max_value() as usize);
        assert!(src_buffer.size() == dst_buffer.size());
        // AddrHigh bit0
        assert!(desc.as_ptr() as usize & 0x01 == 0);

        let transfer_size_bytes = src_buffer.size() as u32;

        let mut config = Config(0);
        config.set_src_drq_port(DrqPort::SdRam);
        config.set_dst_drq_port(DrqPort::SdRam);
        config.set_src_address_mode(AddressMode::Linear);
        config.set_dst_address_mode(AddressMode::Linear);

        // TODO - config or trait provided
        config.set_src_burst_length(BurstLength::Bytes4);
        config.set_dst_burst_length(BurstLength::Bytes4);

        config.set_src_data_width(SrcWord::data_width());
        config.set_dst_data_width(DstWord::data_width());

        let mut param = Param(0);
        param.set_wait(Param::NORMAL_WAIT);

        desc.config = config;
        desc.src_addr = src_buffer.as_ptr() as u32;
        desc.dst_addr = dst_buffer.as_ptr() as u32;
        desc.length = transfer_size_bytes;
        desc.param = param;
        desc.next_addr = Descriptor::LAST_ADDR;

        TransferResources {
            desc,
            src_buffer,
            dst_buffer,
        }
    }

    pub fn free(self) -> (Pin<&'static mut Descriptor>, Pin<SrcBuf>, Pin<DstBuf>) {
        (self.desc, self.src_buffer, self.dst_buffer)
    }
}

#[derive(Debug)]
pub struct Transfer<SrcBuf, DstBuf, State> {
    res: TransferResources<SrcBuf, DstBuf>,
    _state: State,
}

impl<SrcBuf, DstBuf> Transfer<SrcBuf, DstBuf, Ready>
where
    SrcBuf: 'static,
    DstBuf: 'static,
{
    pub fn new<SrcWord, DstWord>(
        res: TransferResources<SrcBuf, DstBuf>,
        channel: &mut Channel,
    ) -> Self
    where
        SrcBuf: Deref,
        SrcBuf::Target: Buffer<SrcWord>,
        DstBuf: DerefMut,
        DstBuf::Target: Buffer<DstWord>,
        SrcWord: SupportedWordSize,
        DstWord: SupportedWordSize,
    {
        channel.set_nonsecure();
        channel.set_desc_addr(&res.desc);

        Transfer { res, _state: Ready }
    }

    pub fn start(self, channel: &mut Channel) -> Transfer<SrcBuf, DstBuf, Started> {
        atomic::fence(Ordering::SeqCst);

        channel.enable();

        Transfer {
            res: self.res,
            _state: Started,
        }
    }
}

impl<SrcBuf, DstBuf> Transfer<SrcBuf, DstBuf, Started> {
    pub fn is_active(&self, channel: &mut Channel) -> bool {
        channel.is_active()
    }

    pub fn wait(self, channel: &mut Channel) -> TransferResources<SrcBuf, DstBuf> {
        // Wait for transfer to finish
        while self.is_active(channel) {
            asm::nop();
        }

        atomic::fence(Ordering::SeqCst);

        self.res
    }
}

impl ChannelNumber {
    fn into_index(self) -> usize {
        match self {
            ChannelNumber::Ch0 => 0,
        }
    }
}

impl Channel {
    fn is_active(&self) -> bool {
        match self.chan_num {
            ChannelNumber::Ch0 => self.dma.status.is_set(Status::Ch0Busy::Read),
        }
    }

    fn enable(&mut self) {
        let chan = self.chan_num.into_index();
        self.dma.channels[chan]
            .enable
            .modify(ChannelEnable::Enable::Set);
    }

    fn set_desc_addr(&mut self, desc: &Descriptor) {
        let addr = desc.as_ptr() as u32;
        let chan = self.chan_num.into_index();
        self.dma.channels[chan].desc_addr.write(addr);
    }

    fn set_nonsecure(&mut self) {
        match self.chan_num {
            ChannelNumber::Ch0 => self.dma.security.modify(Security::Ch0::NonSecure),
        }
    }
}

/// Indicates that a DMA transfer is ready to be started
pub struct Ready;

/// Indicates that a DMA transfer has been started
pub struct Started;

/// Implemented for types that can be used as a buffer for DMA transfers
pub trait Buffer<Word> {
    fn as_ptr(&self) -> *const Word;

    fn len(&self) -> usize;

    fn size(&self) -> usize;
}

impl<T, Word> Buffer<Word> for T
where
    T: ?Sized + AsSlice<Element = Word>,
{
    fn as_ptr(&self) -> *const Word {
        self.as_slice().as_ptr()
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn size(&self) -> usize {
        mem::size_of::<Word>() * self.len()
    }
}

pub trait SupportedWordSize: private::Sealed + Unpin + 'static {
    fn data_width() -> DataWidth;
}

impl private::Sealed for u8 {}
impl SupportedWordSize for u8 {
    fn data_width() -> DataWidth {
        DataWidth::Bits8
    }
}

impl private::Sealed for u16 {}
impl SupportedWordSize for u16 {
    fn data_width() -> DataWidth {
        DataWidth::Bits16
    }
}

impl private::Sealed for u32 {}
impl SupportedWordSize for u32 {
    fn data_width() -> DataWidth {
        DataWidth::Bits32
    }
}

impl private::Sealed for u64 {}
impl SupportedWordSize for u64 {
    fn data_width() -> DataWidth {
        DataWidth::Bits64
    }
}

mod private {
    pub trait Sealed {}
}
