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
    BusClockGating2,
    u32,
    RW,
    Fields [
        Pio WIDTH(U1) OFFSET(U5),
    ]
}

register! {
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
    pub pll_cpu_ctrl: PllCpuXControl::Register,   // 0x0000
    __reserved_0: [u32; 9],                       // 0x0004
    pub pll_periph0: PllPeriph0Control::Register, // 0x0028
    pub pll_periph1: PllPeriph1Control::Register, // 0x002C
    __reserved_1: [u32; 9],                       // 0x0030
    pub ahb1_apb1_cfg: Ahb1Apb1Config::Register,  // 0x0054
    pub apb2_cfg: Apb2Config::Register,           // 0x0058
    pub ahb2_cfg: Ahb2Config::Register,           // 0x005C
    __reserved_2: [u32; 2],                       // 0x0060
    pub bcg2: BusClockGating2::Register,          // 0x0068
    pub bcg3: BusClockGating3::Register,          // 0x006C
    __reserved_3: [u32; 154],                     // 0x0070
    pub bsr4: BusSoftReset4::Register,            // 0x02D8
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
