//! Timer
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C2_0C00;

register! {
    IrqEnable,
    u32,
    RW,
    Fields [
        Timer0IrqEnable WIDTH(U1) OFFSET(U0),
        Timer1IrqEnable WIDTH(U1) OFFSET(U1),
    ]
}

register! {
    IrqStatus,
    u32,
    RW,
    Fields [
        Timer0IrqPending WIDTH(U1) OFFSET(U0),
        Timer1IrqPending WIDTH(U1) OFFSET(U1),
    ]
}

register! {
    Control,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0),
        Reload WIDTH(U1) OFFSET(U1),
        ClockSrc WIDTH(U2) OFFSET(U2) [
            Clock32K = U0,
            Clock24M = U1
        ],
        Prescale WIDTH(U3) OFFSET(U4) [
            Div1 = U0,
            Div2 = U1,
            Div4 = U2,
            Div8 = U3,
            Div16 = U4,
            Div32 = U5,
            Div64 = U6,
            Div128 = U7
        ],
        Mode WIDTH(U1) OFFSET(U7) [
            Continuous = U0,
            OneShot = U1
        ],
    ]
}

register! {
    IntervalValue,
    u32,
    RW,
    Fields [
        Value WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    /// Counts down from IntervalValue to zero, 32-bits
    CurrentValue,
    u32,
    RW,
    Fields [
        Value WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    AvsCounterControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    AvsCounter0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    AvsCounter1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

register! {
    AvsCounterDivider,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0)
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x90);

#[repr(C)]
pub struct RegisterBlock {
    pub irq_enable: IrqEnable::Register,            // 0x00
    pub irq_status: IrqStatus::Register,            // 0x04
    __reserved_0: [u32; 2],                         // 0x08
    pub ctrl0: Control::Register,                   // 0x10
    pub intv0: IntervalValue::Register,             // 0x14
    pub cval0: CurrentValue::Register,              // 0x18
    __reserved_1: [u32; 1],                         // 0x1C
    pub ctrl1: Control::Register,                   // 0x20
    pub intv1: IntervalValue::Register,             // 0x24
    pub cval1: CurrentValue::Register,              // 0x28
    __reserved_2: [u32; 21],                        // 0x2C
    pub avs_cntr_ctrl: AvsCounterControl::Register, // 0x80
    pub avs_cntr0: AvsCounter0::Register,           // 0x84
    pub avs_cntr1: AvsCounter1::Register,           // 0x88
    pub avs_cntr_div: AvsCounterDivider::Register,  // 0x8C
}
// TODO - watchdog0 registers

pub struct TIMER {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMER {}

impl TIMER {
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

impl Deref for TIMER {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for TIMER {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
