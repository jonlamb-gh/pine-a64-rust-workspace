//! Display engine (DE 2.0)
//!
//! Size: 4M

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x0100_0000;

register! {
    GateConfig,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    BusConfig,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    ResetConfig,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    DivConfig,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    SelConfig,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x014);

#[repr(C)]
pub struct RegisterBlock {
    pub gate_cfg: GateConfig::Register, // 0x000
    pub bus_cfg: BusConfig::Register,   // 0x004
    pub rst_cfg: ResetConfig::Register, // 0x008
    pub div_cfg: DivConfig::Register,   // 0x00C
    pub sel_cfg: SelConfig::Register,   // 0x010
}

pub struct DE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DE {}

impl DE {
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

impl Deref for DE {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for DE {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
