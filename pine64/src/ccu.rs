//! CCU
//!
//! Size: 1K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C2_0000;

register! {
    PllCpuXControl,
    u32,
    RO,
    Fields [
        FactorM WIDTH(U2) OFFSET(U0),
        FactorK WIDTH(U2) OFFSET(U4),
        FactorN WIDTH(U5) OFFSET(U8),
        PllOutExtDivP WIDTH(U2) OFFSET(U16) [
            Divide1 = U0,
            Divide2 = U1,
            Divide4 = U2,
            Divide8 = U3
        ]
        Lock WIDTH(U1) OFFSET(U28)
    ]
}

register! {
    PllVideo0Control,
    u32,
    RW,
    Fields [
        PreDivM WIDTH(U4) OFFSET(U0),
        FactorN WIDTH(U7) OFFSET(U8),
        Mode WIDTH(U1) OFFSET(U24) [
            Fractional = U0,
            Integer = U1
        ]
        Lock WIDTH(U1) OFFSET(U28)
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    PllPeriph0Control,
    u32,
    RO,
    Fields [
        FactorK WIDTH(U2) OFFSET(U4),
        FactorN WIDTH(U5) OFFSET(U8),
        Lock WIDTH(U1) OFFSET(U28),
    ]
}

register! {
    PllPeriph1Control,
    u32,
    RO,
    Fields [
        FactorK WIDTH(U2) OFFSET(U4),
        FactorN WIDTH(U5) OFFSET(U8),
        Lock WIDTH(U1) OFFSET(U28),
    ]
}

register! {
    PllDeControl,
    u32,
    RW,
    Fields [
        PreDivM WIDTH(U4) OFFSET(U0),
        FactorN WIDTH(U7) OFFSET(U8),
        PllSdmEn WIDTH(U1) OFFSET(U20),
        Mode WIDTH(U1) OFFSET(U24) [
            Fractional = U0,
            Integer = U1
        ]
        FracClockOut WIDTH(U1) OFFSET(U25),
        Lock WIDTH(U1) OFFSET(U28),
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    Ahb1Apb1Config,
    u32,
    RO,
    Fields [
        Ahb1ClockDivRatio WIDTH(U2) OFFSET(U4) [
            Divide1 = U0,
            Divide2 = U1,
            Divide4 = U2,
            Divide8 = U3
        ]
        Ahb1PreDiv WIDTH(U2) OFFSET(U6),
        Apb1ClockDivRatio WIDTH(U2) OFFSET(U8) [
            Divide2 = U1,
            Divide4 = U2,
            Divide8 = U3
        ]
        Ahb1ClockSrcSel WIDTH(U2) OFFSET(U12) [
            LOsc = U0,
            Osc24M = U1,
            Axi = U2,
            PllPeriph01x = U3
        ]
    ]
}

register! {
    Apb2Config,
    u32,
    RO,
    Fields [
        RatioM WIDTH(U5) OFFSET(U0),
        RatioN WIDTH(U2) OFFSET(U16) [
            Divide1 = U0,
            Divide2 = U1,
            Divide4 = U2,
            Divide8 = U3
        ]
        ClockSrcSel WIDTH(U2) OFFSET(U24) [
            LOsc = U0,
            Osc24M = U1,
            PllPeriph02x = U2
        ]
    ]
}

register! {
    Ahb2Config,
    u32,
    RO,
    Fields [
        ClockConfig WIDTH(U2) OFFSET(U0) [
            Ahb1Clock = U0,
            PllPeriph01xD2 = U1
        ]
    ]
}

register! {
    /// AHB gating 0
    BusClockGating0,
    u32,
    RW,
    Fields [
        Dma WIDTH(U1) OFFSET(U6),
    ]
}

register! {
    /// AHB gating 1
    BusClockGating1,
    u32,
    RW,
    Fields [
        Tcon0 WIDTH(U1) OFFSET(U3),
        Tcon1 WIDTH(U1) OFFSET(U4),
        Hdmi WIDTH(U1) OFFSET(U11),
        De WIDTH(U1) OFFSET(U12),
    ]
}

register! {
    /// APB1 gating
    BusClockGating2,
    u32,
    RW,
    Fields [
        Pio WIDTH(U1) OFFSET(U5),
    ]
}

register! {
    /// APB2 gating
    BusClockGating3,
    u32,
    RW,
    Fields [
        Uart0 WIDTH(U1) OFFSET(U16),
        Uart1 WIDTH(U1) OFFSET(U17),
        Uart2 WIDTH(U1) OFFSET(U18),
        Uart3 WIDTH(U1) OFFSET(U19),
        Uart4 WIDTH(U1) OFFSET(U20)
    ]
}

register! {
    DeClockConfig,
    u32,
    RW,
    Fields [
        DivRatioM WIDTH(U4) OFFSET(U0),
        ClockSel WIDTH(U3) OFFSET(U24) [
            PllPeriph0x2 = U0,
            PllDe = U1
        ]
        SClockGating WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    Tcon1ClockConfig,
    u32,
    RW,
    Fields [
        DivRatioM WIDTH(U4) OFFSET(U0),
        SClockGating WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    HdmiClockConfig,
    u32,
    RW,
    Fields [
        DivRatioM WIDTH(U4) OFFSET(U0),
        ClockSel WIDTH(U2) OFFSET(U24) [
            Pll3Video0x1 = U0,
            Pll7Video1x1 = U1,
            Pll3Video0x2 = U2,
            Pll7Video1x2 = U3
        ]
        SClockGating WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    HdmiSlowClock,
    u32,
    RW,
    Fields [
        DdcClockGating WIDTH(U1) OFFSET(U31),
    ]
}

register! {
    /// AHB1 reset 0
    BusSoftReset0,
    u32,
    RW,
    Fields [
        Dma WIDTH(U1) OFFSET(U6),
    ]
}

register! {
    /// AHB1 reset 1
    BusSoftReset1,
    u32,
    RW,
    Fields [
        Tcon0 WIDTH(U1) OFFSET(U3),
        Tcon1 WIDTH(U1) OFFSET(U4),
        Hdmi0 WIDTH(U1) OFFSET(U10),
        Hdmi1 WIDTH(U1) OFFSET(U11),
        De WIDTH(U1) OFFSET(U12),
    ]
}

register! {
    /// AHB1 reset 2
    BusSoftReset2,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    /// APB1 reset
    BusSoftReset3,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    /// APB2 reset
    BusSoftReset4,
    u32,
    RW,
    Fields [
        Scr WIDTH(U1) OFFSET(U5),
        Uart0 WIDTH(U1) OFFSET(U16),
        Uart1 WIDTH(U1) OFFSET(U17),
        Uart2 WIDTH(U1) OFFSET(U18),
        Uart3 WIDTH(U1) OFFSET(U19),
        Uart4 WIDTH(U1) OFFSET(U20)
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x02DC);

#[repr(C)]
pub struct RegisterBlock {
    pub pll_cpu_ctrl: PllCpuXControl::Register,     // 0x0000
    __reserved_0: [u32; 3],                         // 0x0004
    pub pll_video0: PllVideo0Control::Register,     // 0x0010
    __reserved_1: [u32; 5],                         // 0x0014
    pub pll_periph0: PllPeriph0Control::Register,   // 0x0028
    pub pll_periph1: PllPeriph1Control::Register,   // 0x002C
    __reserved_2: [u32; 6],                         // 0x0030
    pub pll_de: PllDeControl::Register,             // 0x0048
    __reserved_3: [u32; 2],                         // 0x004C
    pub ahb1_apb1_cfg: Ahb1Apb1Config::Register,    // 0x0054
    pub apb2_cfg: Apb2Config::Register,             // 0x0058
    pub ahb2_cfg: Ahb2Config::Register,             // 0x005C
    pub bcg0: BusClockGating0::Register,            // 0x0060
    pub bcg1: BusClockGating1::Register,            // 0x0064
    pub bcg2: BusClockGating2::Register,            // 0x0068
    pub bcg3: BusClockGating3::Register,            // 0x006C
    __reserved_5: [u32; 37],                        // 0x0070
    pub de_clk_cfg: DeClockConfig::Register,        // 0x0104
    __reserved_6: [u32; 5],                         // 0x0108
    pub tcon1_clk_cfg: Tcon1ClockConfig::Register,  // 0x011C
    __reserved_7: [u32; 12],                        // 0x0120
    pub hdmi_clk_cfg: HdmiClockConfig::Register,    // 0x0150
    pub hdmi_slow_clk_cfg: HdmiSlowClock::Register, // 0x0154
    __reserved_8: [u32; 90],                        // 0x0158
    pub bsr0: BusSoftReset0::Register,              // 0x02C0
    pub bsr1: BusSoftReset1::Register,              // 0x02C4
    pub bsr2: BusSoftReset2::Register,              // 0x02C8
    __reserved_9: u32,                              // 0x02CC
    pub bsr3: BusSoftReset3::Register,              // 0x02D0
    __reserved_10: u32,                             // 0x02D4
    pub bsr4: BusSoftReset4::Register,              // 0x02D8
}

pub struct CCU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CCU {}

impl CCU {
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

impl Deref for CCU {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for CCU {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
