//! HDMI
//!
//! Size: 128K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01EE_0000;

const PHY_OFFSET: usize = 0x0001_0000;
pub const PHY_PADDR: usize = PADDR + PHY_OFFSET;

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

register! {
    PhyPol,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyReadEn,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyUnscramble,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyControl,
    u32,
    RW,
    Fields [
        B0 WIDTH(U1) OFFSET(U0),
        B1 WIDTH(U1) OFFSET(U1),
        B2 WIDTH(U1) OFFSET(U2),
        B3 WIDTH(U1) OFFSET(U3),
        F0 WIDTH(U3) OFFSET(U4) [
            Full = U7
        ]
        B7 WIDTH(U1) OFFSET(U7),
        F1 WIDTH(U4) OFFSET(U8) [
            Full = U15
        ]
        F2 WIDTH(U4) OFFSET(U12) [
            Full = U15
        ]
        B16 WIDTH(U1) OFFSET(U16),
        B18 WIDTH(U1) OFFSET(U18),
        B19 WIDTH(U1) OFFSET(U19),
    ]
}

register! {
    PhyUnk1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyUnk2,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyPll,
    u32,
    RW,
    Fields [
        F0 WIDTH(U6) OFFSET(U0),
        B25 WIDTH(U1) OFFSET(U25),
        B30 WIDTH(U1) OFFSET(U30),
        B31 WIDTH(U1) OFFSET(U31),
    ]
}

register! {
    PhyClock,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyUnk3,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PhyStatus,
    u32,
    RW,
    Fields [
        Ready WIDTH(U1) OFFSET(U7),
        PlugIn WIDTH(U1) OFFSET(U19),
    ]
}

register! {
    // Offset 0x01FC..=0x01FF
    Rx1fc,
    u32,
    RW,
    Fields [
        Byte0 WIDTH(U8) OFFSET(U0),
        Byte1 WIDTH(U8) OFFSET(U8),
        Byte2 WIDTH(U8) OFFSET(U16),
        IhMute WIDTH(U8) OFFSET(U24) [
            Disabled = U3
        ]
    ]
}

register! {
    // Offset 0x7E04..=0x7E07
    Rx7e04,
    u32,
    RW,
    Fields [
        I2cmOp WIDTH(U8) OFFSET(U0),
        I2cmInt WIDTH(U8) OFFSET(U8),
        I2cmCtlInt WIDTH(U8) OFFSET(U16),
        I2cmDiv WIDTH(U8) OFFSET(U24),
    ]
}

register! {
    // Offset 0x7E08..=0x7E0B
    Rx7e08,
    u32,
    RW,
    Fields [
        I2cmSegAddr WIDTH(U8) OFFSET(U0),
        I2cmSoftRstz WIDTH(U8) OFFSET(U8),
        I2cmSegPtr WIDTH(U8) OFFSET(U16),
        I2cmSsSclHcnt1Addr WIDTH(U8) OFFSET(U24),
    ]
}

register! {
    // Offset 0x7E0C..=0x7E0F
    Rx7e0c,
    u32,
    RW,
    Fields [
        I2cmSsSclHcnt0Addr WIDTH(U8) OFFSET(U0),
        I2cmSsSclLcnt1Addr WIDTH(U8) OFFSET(U8),
        I2cmSsSclLcnt0Addr WIDTH(U8) OFFSET(U16),
        I2cmFsSclHcnt1Addr WIDTH(U8) OFFSET(U24),
    ]
}

register! {
    // Offset 0x7E10..=0x7E13
    Rx7e10,
    u32,
    RW,
    Fields [
        I2cmFsSclHcnt0Addr WIDTH(U8) OFFSET(U0),
        I2cmFsSclLcnt1Addr WIDTH(U8) OFFSET(U8),
        I2cmFsSclLcnt0Addr WIDTH(U8) OFFSET(U16),
        Byte3 WIDTH(U8) OFFSET(U24),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x0001_003C);

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
    __reserved_3: [u32; 69],                            // 0x0E8
    pub r1fc: Rx1fc::Register,                          // 0x1FC
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
    __reserved_11: [u32; 7712],                         // 0x584
    pub r7e04: Rx7e04::Register,                        // 0x7E04
    pub r7e08: Rx7e08::Register,                        // 0x7E08
    pub r7e0c: Rx7e0c::Register,                        // 0x7E0C
    pub r7e10: Rx7e10::Register,                        // 0x7E10
    __reserved_12: [u32; 8315],                         // 0x7E14
    pub phy_pol: PhyPol::Register,                      // 0x1_0000
    __reserved_13: [u32; 3],                            // 0x1_0004
    pub phy_read_en: PhyReadEn::Register,               // 0x1_0010
    pub phy_unscramble: PhyUnscramble::Register,        // 0x1_0014
    __reserved_14: [u32; 2],                            // 0x1_0018
    pub phy_ctrl: PhyControl::Register,                 // 0x1_0020
    pub phy_unk1: PhyUnk1::Register,                    // 0x1_0024
    pub phy_unk2: PhyUnk2::Register,                    // 0x1_0028
    pub phy_pll: PhyPll::Register,                      // 0x1_002C
    pub phy_clk: PhyClock::Register,                    // 0x1_0030
    pub phy_unk3: PhyUnk3::Register,                    // 0x1_0034
    pub phy_status: PhyStatus::Register,                // 0x1_0038
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
