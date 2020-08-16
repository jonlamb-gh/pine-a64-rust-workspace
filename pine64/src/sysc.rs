//! System control
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C0_0000;

register! {
    SRamCtrl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x08);

#[repr(C)]
pub struct RegisterBlock {
    pub __reserved_0: u32,             // 0x00
    pub sram_ctrl: SRamCtrl::Register, // 0x04
}

pub struct SYSC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSC {}

impl SYSC {
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

impl Deref for SYSC {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for SYSC {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
