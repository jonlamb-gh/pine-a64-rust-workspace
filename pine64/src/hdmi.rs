//! HDMI
//!
//! Size: 128K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01EE_0000;

register! {
    VersionId,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Control,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U31),
    ]
}

register! {
    Interrupt,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Hpd,
    u32,
    RW,
    Fields [
        Detect WIDTH(U1) OFFSET(U0),
    ]
}

register! {
    VideoControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    VideoSize,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    VideoBp,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    VideoFp,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    VideoSpw,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    VideoPolarity,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    QcpPacket0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    QcpPacket1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PadControl0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PadControl1,
    u32,
    RW,
    Fields [
        Halve WIDTH(U1) OFFSET(U6),
    ]
}

register! {
    PllControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PllDbg0,
    u32,
    RW,
    Fields [
        Pll WIDTH(U1) OFFSET(U21) [
            Pll3Video0 = U0,
            Pll7Video1 = U1
        ]
    ]
}

register! {
    PllDbg1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    HpdCec,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PacketControl0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PacketControl1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    AudioSampleCount,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    AudioTxFifo,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcExreg,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcCommand,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcAddress,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcIntMask,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcIntStatus,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcFifoControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcFifoStatus,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcClock,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcTimeout,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DdcFifoData,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x584);

#[repr(C)]
pub struct RegisterBlock {
    pub version: VersionId::Register,                   // 0x000
    pub ctrl: Control::Register,                        // 0x004
    pub irq: Interrupt::Register,                       // 0x008
    pub hpd: Hpd::Register,                             // 0x00C
    pub video_ctrl: VideoControl::Register,             // 0x010
    pub video_size: VideoSize::Register,                // 0x014
    pub video_bp: VideoBp::Register,                    // 0x018
    pub video_fp: VideoFp::Register,                    // 0x01C
    pub video_spw: VideoSpw::Register,                  // 0x020
    pub video_polarity: VideoPolarity::Register,        // 0x024
    __reserved_0: [u32; 22],                            // 0x028
    pub avi_info_frame: [u32; 5],                       // 0x080
    __reserved_2: [u32; 19],                            // 0x094
    pub qcp_packet0: QcpPacket0::Register,              // 0x0E0
    pub qcp_packet1: QcpPacket1::Register,              // 0x0E4
    __reserved_3: [u32; 70],                            // 0x0E8
    pub pad_ctrl0: PadControl0::Register,               // 0x200
    pub pad_ctrl1: PadControl1::Register,               // 0x204
    pub pll_ctrl: PllControl::Register,                 // 0x208
    pub pll_dbg0: PllDbg0::Register,                    // 0x20C
    pub pll_dbg1: PllDbg1::Register,                    // 0x210
    pub hpd_cec: HpdCec::Register,                      // 0x214
    __reserved_5: [u32; 10],                            // 0x218
    pub vendor_info_frame: [u32; 5],                    // 0x240
    __reserved_6: [u32; 39],                            // 0x254
    pub pkt_ctrl0: PacketControl0::Register,            // 0x2F0
    pub pkt_ctrl1: PacketControl1::Register,            // 0x2F4
    __reserved_7: [u32; 6],                             // 0x2F8
    pub audio_sample_count: AudioSampleCount::Register, // 0x310
    __reserved_8: [u32; 59],                            // 0x314
    pub audio_tx_fifo: AudioTxFifo::Register,           // 0x400
    __reserved_9: [u32; 63],                            // 0x404
    pub ddc_ctrl: DdcControl::Register,                 // 0x500
    pub ddc_exreg: DdcExreg::Register,                  // 0x504
    pub ddc_cmd: DdcCommand::Register,                  // 0x508
    pub ddc_addr: DdcAddress::Register,                 // 0x50C
    pub ddc_int_mask: DdcIntMask::Register,             // 0x510
    pub ddc_int_status: DdcIntStatus::Register,         // 0x514
    pub ddc_fifo_ctrl: DdcFifoControl::Register,        // 0x518
    pub ddc_fifo_status: DdcFifoStatus::Register,       // 0x51C
    pub ddc_clock: DdcClock::Register,                  // 0x520
    pub ddc_timeout: DdcTimeout::Register,              // 0x524
    __reserved_10: [u32; 22],                           // 0x528
    pub ddc_fifo_data: DdcFifoData::Register,           // 0x580
}

pub struct HDMI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HDMI {}

impl HDMI {
    pub unsafe fn from_paddr() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const RegisterBlock {
        PADDR as *const _
    }

    pub const unsafe fn ptr() -> *const RegisterBlock {
        PADDR as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut RegisterBlock {
        PADDR as *mut _
    }

    pub const unsafe fn mut_ptr() -> *mut RegisterBlock {
        PADDR as *mut _
    }
}

impl Deref for HDMI {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for HDMI {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
