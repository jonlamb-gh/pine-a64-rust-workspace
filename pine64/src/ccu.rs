//! CCU
//!
//! Size: 1K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C2_0000;

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
    __reserved_0: [u32; 26],             // 0x0000
    pub bcg2: BusClockGating2::Register, // 0x0068
    pub bcg3: BusClockGating3::Register, // 0x006C
    __reserved_1: [u32; 154],            // 0x0070
    pub bsr4: BusSoftReset4::Register,   // 0x02D8
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
