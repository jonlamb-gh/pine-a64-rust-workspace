//! TCON0
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C0_C000;

register! {
    GlobalControl,
    u32,
    RW,
    Fields [
        IoMapSelect WIDTH(U1) OFFSET(U0),
        GammaEnable WIDTH(U1) OFFSET(U30),
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    GlobalInt0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    GlobalInt1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    FrmControl,
    u32,
    RW,
    Fields [
        ModeB WIDTH(U1) OFFSET(U4) [
            SixBit = U0,
            FiveBit = U1
        ]
        ModeG WIDTH(U1) OFFSET(U5) [
            SixBit = U0,
            FiveBit = U1
        ]
        ModeR WIDTH(U1) OFFSET(U6) [
            SixBit = U0,
            FiveBit = U1
        ]
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    FrmSeed,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    FrmTable,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Fifo3d,
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
        SrcSel WIDTH(U3) OFFSET(U0),
        StartDelay WIDTH(U5) OFFSET(U4),
        Fifo1Reset WIDTH(U1) OFFSET(U21),
        RBSwap WIDTH(U1) OFFSET(U23),
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    DataClock,
    u32,
    RW,
    Fields [
        Divider WIDTH(U7) OFFSET(U0),
        Enable WIDTH(U4) OFFSET(U28)
    ]
}

register! {
    Timing0,
    u32,
    RW,
    Fields [
        Y WIDTH(U12) OFFSET(U0),
        X WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing1,
    u32,
    RW,
    Fields [
        HorizBackPorch WIDTH(U12) OFFSET(U0),
        Ht WIDTH(U13) OFFSET(U16),
    ]
}

register! {
    Timing2,
    u32,
    RW,
    Fields [
        VertBackPorch WIDTH(U12) OFFSET(U0),
        Vt WIDTH(U13) OFFSET(U16),
    ]
}

register! {
    Timing3,
    u32,
    RW,
    Fields [
        Vspw WIDTH(U10) OFFSET(U0),
        Hspw WIDTH(U10) OFFSET(U16),
    ]
}

register! {
    HvIface,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuIface,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuWriteData,
    u32,
    WO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuReadData0,
    u32,
    RO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuReadData1,
    u32,
    RO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    LvdsIface,
    u32,
    RW,
    Fields [
        DataPolarity WIDTH(U4) OFFSET(U0),
        ClockPolarity WIDTH(U1) OFFSET(U4),
        ClockSelect WIDTH(U1) OFFSET(U20),
        CorrectMode WIDTH(U1) OFFSET(U23),
        DebugMode WIDTH(U1) OFFSET(U24),
        BitWidth WIDTH(U1) OFFSET(U26) [
            Bits24 = U0,
            Bits18 = U1
        ]
        Mode WIDTH(U1) OFFSET(U27) [
            Ns = U0,
            Jeida = U1
        ]
        Direction WIDTH(U1) OFFSET(U28) [
            Normal = U0,
            Reverse = U1
        ]
        EvenOddDirection WIDTH(U1) OFFSET(U29) [
            Normal = U0,
            Reverse = U1
        ]
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    IoPolarity,
    u32,
    RW,
    Fields [
        DataInvert WIDTH(U24) OFFSET(U0),
        Io0Invert WIDTH(U1) OFFSET(U24),
        Io1Invert WIDTH(U1) OFFSET(U25),
        Io2Invert WIDTH(U1) OFFSET(U26),
        Io3Invert WIDTH(U1) OFFSET(U27),
        DataClockSelect WIDTH(U3) OFFSET(U28),
        IoOutputSelect WIDTH(U1) OFFSET(U31),
    ]
}

register! {
    IoTristate,
    u32,
    RW,
    Fields [
        DataOutputTriEnable WIDTH(U24) OFFSET(U0),
        Io0OutputTriEnable WIDTH(U1) OFFSET(U24),
        Io1OutputTriEnable WIDTH(U1) OFFSET(U25),
        Io2OutputTriEnable WIDTH(U1) OFFSET(U26),
        Io3OutputTriEnable WIDTH(U1) OFFSET(U27),
        RgbEndian WIDTH(U1) OFFSET(U28),
    ]
}

register! {
    LvdsAnalog0,
    u32,
    RW,
    Fields [
        RegPwSmb WIDTH(U1) OFFSET(U0),
        RegPwSlv WIDTH(U1) OFFSET(U1),
        RegPd WIDTH(U2) OFFSET(U4),
        RegV WIDTH(U2) OFFSET(U8),
        RegDen WIDTH(U4) OFFSET(U12),
        RegDenC WIDTH(U1) OFFSET(U16),
        RegC WIDTH(U2) OFFSET(U17),
        RegDRamTest WIDTH(U1) OFFSET(U19),
        EnDrv WIDTH(U4) OFFSET(U20),
        EnDrvC WIDTH(U1) OFFSET(U24),
        EnLdo WIDTH(U1) OFFSET(U30),
        EnMb WIDTH(U1) OFFSET(U31),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x224);

#[repr(C)]
pub struct RegisterBlock {
    pub gctrl: GlobalControl::Register,      // 0x000
    pub gint0: GlobalInt0::Register,         // 0x004
    pub gint1: GlobalInt1::Register,         // 0x008
    __reserved_0: u32,                       // 0x00C
    pub frm_ctrl: FrmControl::Register,      // 0x010
    pub frm_seed: [FrmSeed::Register; 6],    // 0x014
    pub frm_table: [FrmTable::Register; 4],  // 0x02C
    pub fifo_3d: Fifo3d::Register,           // 0x03C
    pub ctrl: Control::Register,             // 0x040
    pub dclk: DataClock::Register,           // 0x044
    pub timing_active: Timing0::Register,    // 0x048
    pub timing_h: Timing1::Register,         // 0x04C
    pub timing_v: Timing2::Register,         // 0x050
    pub timing_sync: Timing3::Register,      // 0x054
    pub hv_iface: HvIface::Register,         // 0x058
    __reserved_1: u32,                       // 0x05C
    pub cpu_iface: CpuIface::Register,       // 0x060
    pub cpu_wdata: CpuWriteData::Register,   // 0x064
    pub cpu_rdata0: CpuReadData0::Register,  // 0x068
    pub cpu_rdata1: CpuReadData1::Register,  // 0x06C
    __reserved_2: [u32; 5],                  // 0x070
    pub lvds_iface: LvdsIface::Register,     // 0x084
    pub io_polarity: IoPolarity::Register,   // 0x088
    pub io_tristate: IoTristate::Register,   // 0x08C
    __reserved_3: [u32; 100],                // 0x090
    pub lvds_analog0: LvdsAnalog0::Register, // 0x220
}

pub struct TCON0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TCON0 {}

impl TCON0 {
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

impl Deref for TCON0 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for TCON0 {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
