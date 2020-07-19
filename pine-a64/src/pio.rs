//! Port controller (PIO)
//!
//! Size: 1K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C2_0800;

register! {
    PortBConfig0,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Uart2Tx = U3,
            JtagMs0 = U4,
            PortBExtInt0 = U6,
            Disabled = U7
        ]
        Pin1 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Uart2Rx = U3,
            JtagCk0 = U4,
            SimPwrEn = U5,
            PortBExtInt1 = U6,
            Disabled = U7
        ]
        Pin2 WIDTH(U3) OFFSET(U8),
        Pin3 WIDTH(U3) OFFSET(U12),
        Pin4 WIDTH(U3) OFFSET(U16),
        Pin5 WIDTH(U3) OFFSET(U20),
        Pin6 WIDTH(U3) OFFSET(U24),
        Pin7 WIDTH(U3) OFFSET(U28),
    ]
}

register! {
    PortBConfig1,
    u32,
    RW,
    Fields [
        Pin8 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Uart0Tx = U4,
            PortBExtInt8 = U6,
            Disabled = U7
        ]
        Pin9 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Uart0Rx = U4,
            SimPwrEn = U5,
            PortBExtInt9 = U6,
            Disabled = U7
        ]
    ]
}

// Registers that are common with all ports below

register! {
    Config0,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin1 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin2 WIDTH(U3) OFFSET(U8) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin3 WIDTH(U3) OFFSET(U12) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin4 WIDTH(U3) OFFSET(U16) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin5 WIDTH(U3) OFFSET(U20) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin6 WIDTH(U3) OFFSET(U24) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin7 WIDTH(U3) OFFSET(U28) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
    ]
}

register! {
    Config1,
    u32,
    RW,
    Fields [
        Pin8 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
        Pin9 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
    ]
}

register! {
    Config2,
    u32,
    RW,
    Fields [
        Pin16 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
    ]
}

register! {
    Config3,
    u32,
    RW,
    Fields [
        Pin24 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Disabled = U7
        ]
    ]
}

register! {
    Data,
    u32,
    RW,
    Fields [
        Data WIDTH(U8) OFFSET(U0)
    ]
}

register! {
    Driv0,
    u32,
    RW,
    Fields [
        Data WIDTH(U20) OFFSET(U0)
    ]
}

register! {
    Driv1,
    u32,
    RW,
    Fields [
        Data WIDTH(U20) OFFSET(U0)
    ]
}

register! {
    Pull0,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U2) OFFSET(U0) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin1 WIDTH(U2) OFFSET(U2) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin2 WIDTH(U2) OFFSET(U4) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin3 WIDTH(U2) OFFSET(U6) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin4 WIDTH(U2) OFFSET(U8) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin5 WIDTH(U2) OFFSET(U10) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin6 WIDTH(U2) OFFSET(U12) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin7 WIDTH(U2) OFFSET(U14) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin8 WIDTH(U2) OFFSET(U16) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin9 WIDTH(U2) OFFSET(U18) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
    ]
}

register! {
    Pull1,
    u32,
    RW,
    Fields [
        Pin16 WIDTH(U2) OFFSET(U0) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x90);

#[repr(C)]
pub struct RegisterBlock {
    __reserved_0: [u32; 9],              // 0x00
    pub pb_cfg0: PortBConfig0::Register, // 0x24
    pub pb_cfg1: PortBConfig1::Register, // 0x28
    __reserved_1: u32,                   // 0x2C
    __reserved_2: u32,                   // 0x30
    pub pb_data: Data::Register,         // 0x34
    pub pb_driv0: Driv0::Register,       // 0x38
    __reserved_3: u32,                   // 0x3C
    pub pb_pull0: Pull0::Register,       // 0x40
    __reserved_4: u32,                   // 0x44
    pub pc_cfg0: Config0::Register,      // 0x48
    pub pc_cfg1: Config1::Register,      // 0x4C
    pub pc_cfg2: Config2::Register,      // 0x50
    __reserved_5: u32,                   // 0x54
    pub pc_data: Data::Register,         // 0x58
    pub pc_driv0: Driv0::Register,       // 0x5C
    pub pc_driv1: Driv1::Register,       // 0x60
    pub pc_pull0: Pull0::Register,       // 0x64
    pub pc_pull1: Pull1::Register,       // 0x68
    pub pd_cfg0: Config0::Register,      // 0x6C
    pub pd_cfg1: Config1::Register,      // 0x70
    pub pd_cfg2: Config2::Register,      // 0x74
    pub pd_cfg3: Config3::Register,      // 0x78
    pub pd_data: Data::Register,         // 0x7C
    pub pd_driv0: Driv0::Register,       // 0x80
    pub pd_driv1: Driv1::Register,       // 0x84
    pub pd_pull0: Pull0::Register,       // 0x88
    pub pd_pull1: Pull1::Register,       // 0x8C
}

pub struct PIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO {}

impl PIO {
    pub unsafe fn from_paddr() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const RegisterBlock {
        PADDR as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut RegisterBlock {
        PADDR as *mut _
    }
}

impl Deref for PIO {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for PIO {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
