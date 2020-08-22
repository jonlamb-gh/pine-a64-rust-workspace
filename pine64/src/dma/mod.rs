//! DMA
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub mod channel;

pub const PADDR: usize = 0x01C0_2000;

pub const NUM_CHANNELS: usize = 8;

register! {
    IrqEnable,
    u32,
    RW,
    Fields [
        Ch0HalfPkgIrqEnable WIDTH(U1) OFFSET(U0),
        Ch0PkgIrqEnable WIDTH(U1) OFFSET(U1),
        Ch0QueueIrqEnable WIDTH(U1) OFFSET(U2),
        Ch1HalfPkgIrqEnable WIDTH(U1) OFFSET(U4),
        Ch1PkgIrqEnable WIDTH(U1) OFFSET(U5),
        Ch1QueueIrqEnable WIDTH(U1) OFFSET(U6),
        Ch2HalfPkgIrqEnable WIDTH(U1) OFFSET(U8),
        Ch2PkgIrqEnable WIDTH(U1) OFFSET(U9),
        Ch2QueueIrqEnable WIDTH(U1) OFFSET(U10),
        Ch3HalfPkgIrqEnable WIDTH(U1) OFFSET(U12),
        Ch3PkgIrqEnable WIDTH(U1) OFFSET(U13),
        Ch3QueueIrqEnable WIDTH(U1) OFFSET(U14),
        Ch4HalfPkgIrqEnable WIDTH(U1) OFFSET(U16),
        Ch4PkgIrqEnable WIDTH(U1) OFFSET(U17),
        Ch4QueueIrqEnable WIDTH(U1) OFFSET(U18),
        Ch5HalfPkgIrqEnable WIDTH(U1) OFFSET(U20),
        Ch5PkgIrqEnable WIDTH(U1) OFFSET(U21),
        Ch5QueueIrqEnable WIDTH(U1) OFFSET(U22),
        Ch6HalfPkgIrqEnable WIDTH(U1) OFFSET(U24),
        Ch6PkgIrqEnable WIDTH(U1) OFFSET(U25),
        Ch6QueueIrqEnable WIDTH(U1) OFFSET(U26),
        Ch7HalfPkgIrqEnable WIDTH(U1) OFFSET(U28),
        Ch7PkgIrqEnable WIDTH(U1) OFFSET(U29),
        Ch7QueueIrqEnable WIDTH(U1) OFFSET(U30),
    ]
}

register! {
    IrqPending,
    u32,
    RW,
    Fields [
        Ch0HalfPkg WIDTH(U1) OFFSET(U0),
        Ch0Pkg WIDTH(U1) OFFSET(U1),
        Ch0Queue WIDTH(U1) OFFSET(U2),
        Ch1HalfPkg WIDTH(U1) OFFSET(U4),
        Ch1Pkg WIDTH(U1) OFFSET(U5),
        Ch1Queue WIDTH(U1) OFFSET(U6),
        Ch2HalfPkg WIDTH(U1) OFFSET(U8),
        Ch2Pkg WIDTH(U1) OFFSET(U9),
        Ch2Queue WIDTH(U1) OFFSET(U10),
        Ch3HalfPkg WIDTH(U1) OFFSET(U12),
        Ch3Pkg WIDTH(U1) OFFSET(U13),
        Ch3Queue WIDTH(U1) OFFSET(U14),
        Ch4HalfPkg WIDTH(U1) OFFSET(U16),
        Ch4Pkg WIDTH(U1) OFFSET(U17),
        Ch4Queue WIDTH(U1) OFFSET(U18),
        Ch5HalfPkg WIDTH(U1) OFFSET(U20),
        Ch5Pkg WIDTH(U1) OFFSET(U21),
        Ch5Queue WIDTH(U1) OFFSET(U22),
        Ch6HalfPkg WIDTH(U1) OFFSET(U24),
        Ch6Pkg WIDTH(U1) OFFSET(U25),
        Ch6Queue WIDTH(U1) OFFSET(U26),
        Ch7HalfPkg WIDTH(U1) OFFSET(U28),
        Ch7Pkg WIDTH(U1) OFFSET(U29),
        Ch7Queue WIDTH(U1) OFFSET(U30),
    ]
}

register! {
    Security,
    u32,
    RW,
    Fields [
        Ch0 WIDTH(U1) OFFSET(U0) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch1 WIDTH(U1) OFFSET(U1) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch2 WIDTH(U1) OFFSET(U2) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch3 WIDTH(U1) OFFSET(U3) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch4 WIDTH(U1) OFFSET(U4) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch5 WIDTH(U1) OFFSET(U5) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch6 WIDTH(U1) OFFSET(U6) [
            Secure = U0,
            NonSecure = U1
        ],
        Ch7 WIDTH(U1) OFFSET(U7) [
            Secure = U0,
            NonSecure = U1
        ],
    ]
}

register! {
    AutoGating,
    u32,
    RW,
    Fields [
        Channel WIDTH(U1) OFFSET(U0) [
            Enable = U0,
            Disable = U1
        ],
        Common WIDTH(U1) OFFSET(U1) [
            Enable = U0,
            Disable = U1
        ],
        MasterClock WIDTH(U1) OFFSET(U2) [
            Enable = U0,
            Disable = U1
        ],
    ]
}

register! {
    Status,
    u32,
    RO,
    Fields [
        Ch0Busy WIDTH(U1) OFFSET(U0),
        Ch1Busy WIDTH(U1) OFFSET(U1),
        Ch2Busy WIDTH(U1) OFFSET(U2),
        Ch3Busy WIDTH(U1) OFFSET(U3),
        Ch4Busy WIDTH(U1) OFFSET(U4),
        Ch5Busy WIDTH(U1) OFFSET(U5),
        Ch6Busy WIDTH(U1) OFFSET(U6),
        Ch7Busy WIDTH(U1) OFFSET(U7),
        MBusFifoNotEmpty WIDTH(U1) OFFSET(U30),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x300);

#[repr(C)]
pub struct RegisterBlock {
    pub irq: IrqEnable::Register,                         // 0x000
    __reserved_0: [u32; 3],                               // 0x004
    pub irq_pending: IrqPending::Register,                // 0x010
    __reserved_1: [u32; 3],                               // 0x014
    pub security: Security::Register,                     // 0x020
    __reserved_2: u32,                                    // 0x024
    pub auto_gating: AutoGating::Register,                // 0x028
    __reserved_3: u32,                                    // 0x02C
    pub status: Status::Register,                         // 0x030
    __reserved_4: [u32; 51],                              // 0x034
    pub channels: [channel::RegisterBlock; NUM_CHANNELS], // 0x100
}

pub struct DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DMA {}

impl DMA {
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

impl Deref for DMA {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for DMA {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
