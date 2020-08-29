//! High-speed timer
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C6_0000;

register! {
    IrqEnable,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0)
    ]
}

register! {
    IrqStatus,
    u32,
    RW,
    Fields [
        IrqPending WIDTH(U1) OFFSET(U0)
    ]
}

register! {
    Control,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0),
        Reload WIDTH(U1) OFFSET(U1),
        Prescale WIDTH(U3) OFFSET(U4) [
            Div1 = U0,
            Div2 = U1,
            Div4 = U2,
            Div8 = U3,
            Div16 = U4
        ],
        Mode WIDTH(U1) OFFSET(U7) [
            Continuous = U0,
            OneShot = U1
        ]
    ]
}

register! {
    IntervalLow,
    u32,
    RW,
    Fields [
        Value WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    IntervalHigh,
    u32,
    RW,
    Fields [
        Value WIDTH(U24) OFFSET(U0)
    ]
}

register! {
    CurrentLow,
    u32,
    RW,
    Fields [
        Value WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    CurrentHigh,
    u32,
    RW,
    Fields [
        Value WIDTH(U24) OFFSET(U0)
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x24);

#[repr(C)]
pub struct RegisterBlock {
    pub irq_enable: IrqEnable::Register, // 0x00
    pub irq_status: IrqStatus::Register, // 0x04
    __reserved_0: [u32; 2],              // 0x08
    pub ctrl: Control::Register,         // 0x10
    pub intv_lo: IntervalLow::Register,  // 0x14
    pub intv_hi: IntervalHigh::Register, // 0x18
    pub cval_lo: CurrentLow::Register,   // 0x1C
    pub cval_hi: CurrentHigh::Register,  // 0x20
}

pub struct HSTIMER {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HSTIMER {}

impl HSTIMER {
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

impl Deref for HSTIMER {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for HSTIMER {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
