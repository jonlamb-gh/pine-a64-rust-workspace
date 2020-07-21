//! Port controller (PIO)
//!
//! Size: 1K
//!
//! PxY_Select variants mapped to alt functions:
//!   * 000 (U0): input
//!   * 001 (U1): output
//!   * 010 (U2): AF0
//!   * 011 (U3): AF1
//!   * 100 (U4): AF2
//!   * 101 (U5): AF3
//!   * 110 (U6): AF4
//!   * 111 (U7): disabled

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C2_0800;

register! {
    Config0,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U3) OFFSET(U0) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin1 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin2 WIDTH(U3) OFFSET(U8) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin3 WIDTH(U3) OFFSET(U12) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin4 WIDTH(U3) OFFSET(U16) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin5 WIDTH(U3) OFFSET(U20) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin6 WIDTH(U3) OFFSET(U24) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin7 WIDTH(U3) OFFSET(U28) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
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
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin9 WIDTH(U3) OFFSET(U4) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin10 WIDTH(U3) OFFSET(U8) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin11 WIDTH(U3) OFFSET(U12) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin12 WIDTH(U3) OFFSET(U16) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin13 WIDTH(U3) OFFSET(U20) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin14 WIDTH(U3) OFFSET(U24) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
        Pin15 WIDTH(U3) OFFSET(U28) [
            Input = U0,
            Output = U1,
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
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
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
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
            Af0 = U2,
            Af1 = U3,
            Af2 = U4,
            Af3 = U5,
            Af4 = U6,
            Disabled = U7
        ]
    ]
}

register! {
    Data,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U1) OFFSET(U0),
        Pin1 WIDTH(U1) OFFSET(U1),
        Pin2 WIDTH(U1) OFFSET(U2),
        Pin3 WIDTH(U1) OFFSET(U3),
        Pin4 WIDTH(U1) OFFSET(U4),
        Pin5 WIDTH(U1) OFFSET(U5),
        Pin6 WIDTH(U1) OFFSET(U6),
        Pin7 WIDTH(U1) OFFSET(U7),
        Pin8 WIDTH(U1) OFFSET(U8),
        Pin9 WIDTH(U1) OFFSET(U9),
        Pin10 WIDTH(U1) OFFSET(U10),
        Pin11 WIDTH(U1) OFFSET(U11),
        Pin12 WIDTH(U1) OFFSET(U12),
        Pin13 WIDTH(U1) OFFSET(U13),
        Pin14 WIDTH(U1) OFFSET(U14),
        Pin15 WIDTH(U1) OFFSET(U15),
        Pin16 WIDTH(U1) OFFSET(U16),
        Pin17 WIDTH(U1) OFFSET(U17),
        Pin18 WIDTH(U1) OFFSET(U18),
        Pin19 WIDTH(U1) OFFSET(U19),
        Pin20 WIDTH(U1) OFFSET(U20),
        Pin21 WIDTH(U1) OFFSET(U21),
        Pin22 WIDTH(U1) OFFSET(U22),
        Pin23 WIDTH(U1) OFFSET(U23),
        Pin24 WIDTH(U1) OFFSET(U24),
    ]
}

register! {
    Driv0,
    u32,
    RW,
    Fields [
        Pin0 WIDTH(U2) OFFSET(U0) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin1 WIDTH(U2) OFFSET(U2) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin2 WIDTH(U2) OFFSET(U4) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin3 WIDTH(U2) OFFSET(U6) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin4 WIDTH(U2) OFFSET(U8) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin5 WIDTH(U2) OFFSET(U10) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin6 WIDTH(U2) OFFSET(U12) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin7 WIDTH(U2) OFFSET(U14) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin8 WIDTH(U2) OFFSET(U16) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin9 WIDTH(U2) OFFSET(U18) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin10 WIDTH(U2) OFFSET(U20) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin11 WIDTH(U2) OFFSET(U22) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin12 WIDTH(U2) OFFSET(U24) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin13 WIDTH(U2) OFFSET(U26) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin14 WIDTH(U2) OFFSET(U28) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
        Pin15 WIDTH(U2) OFFSET(U30) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
    ]
}

register! {
    Driv1,
    u32,
    RW,
    Fields [
        Pin16 WIDTH(U2) OFFSET(U0) [
            Level0 = U0,
            Level1 = U1,
            Level2 = U2,
            Level3 = U3
        ]
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
        Pin10 WIDTH(U2) OFFSET(U20) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin11 WIDTH(U2) OFFSET(U22) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin12 WIDTH(U2) OFFSET(U24) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin13 WIDTH(U2) OFFSET(U26) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin14 WIDTH(U2) OFFSET(U28) [
            Disabled = U0,
            PullUp = U1,
            PullDown = U2
        ]
        Pin15 WIDTH(U2) OFFSET(U30) [
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
    __reserved_0: [u32; 9],         // 0x00
    pub pb_cfg0: Config0::Register, // 0x24
    pub pb_cfg1: Config1::Register, // 0x28
    __reserved_1: u32,              // 0x2C
    __reserved_2: u32,              // 0x30
    pub pb_data: Data::Register,    // 0x34
    pub pb_driv0: Driv0::Register,  // 0x38
    __reserved_3: u32,              // 0x3C
    pub pb_pull0: Pull0::Register,  // 0x40
    __reserved_4: u32,              // 0x44
    pub pc_cfg0: Config0::Register, // 0x48
    pub pc_cfg1: Config1::Register, // 0x4C
    pub pc_cfg2: Config2::Register, // 0x50
    __reserved_5: u32,              // 0x54
    pub pc_data: Data::Register,    // 0x58
    pub pc_driv0: Driv0::Register,  // 0x5C
    pub pc_driv1: Driv1::Register,  // 0x60
    pub pc_pull0: Pull0::Register,  // 0x64
    pub pc_pull1: Pull1::Register,  // 0x68
    pub pd_cfg0: Config0::Register, // 0x6C
    pub pd_cfg1: Config1::Register, // 0x70
    pub pd_cfg2: Config2::Register, // 0x74
    pub pd_cfg3: Config3::Register, // 0x78
    pub pd_data: Data::Register,    // 0x7C
    pub pd_driv0: Driv0::Register,  // 0x80
    pub pd_driv1: Driv1::Register,  // 0x84
    pub pd_pull0: Pull0::Register,  // 0x88
    pub pd_pull1: Pull1::Register,  // 0x8C
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
