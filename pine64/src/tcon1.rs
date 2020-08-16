//! TCON1
//!
//! Size: 4K

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C0_D000;

register! {
    GlobalControl,
    u32,
    RW,
    Fields [
        IoMapSel WIDTH(U1) OFFSET(U0) [
            Tcon0 = U0,
            Tcon1 = U1
        ]
        GammaEnable WIDTH(U1) OFFSET(U30),
        Enable WIDTH(U1) OFFSET(U31)
    ]
}

register! {
    GlobalInt0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    GlobalInt1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Control,
    u32,
    RW,
    Fields [
        StartDelay WIDTH(U5) OFFSET(U4),
        Enable WIDTH(U1) OFFSET(U31),
    ]
}

register! {
    Timing0,
    u32,
    RW,
    Fields [
        Y WIDTH(U12) OFFSET(U0),
        X WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing1,
    u32,
    RW,
    Fields [
        Y WIDTH(U12) OFFSET(U0),
        X WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing2,
    u32,
    RW,
    Fields [
        Y WIDTH(U12) OFFSET(U0),
        X WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing3,
    u32,
    RW,
    Fields [
        Hbp WIDTH(U12) OFFSET(U0),
        Ht WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing4,
    u32,
    RW,
    Fields [
        Vbp WIDTH(U12) OFFSET(U0),
        Vt WIDTH(U12) OFFSET(U16),
    ]
}

register! {
    Timing5,
    u32,
    RW,
    Fields [
        Vspw WIDTH(U10) OFFSET(U0),
        Hspw WIDTH(U10) OFFSET(U16),
    ]
}

register! {
    IoPolarity,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    IoTrigger,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    DataClock,
    u32,
    RW,
    Fields [
        Divider WIDTH(U7) OFFSET(U0),
        Enable WIDTH(U4) OFFSET(U28) [
            Disabled = U0
        ]
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x0F8);

#[repr(C)]
pub struct RegisterBlock {
    pub gctrl: GlobalControl::Register,        // 0x000
    pub gint0: GlobalInt0::Register,           // 0x004
    pub gint1: GlobalInt1::Register,           // 0x008
    __reserved_0: [u32; 14],                   // 0x00C
    pub tcon0_dclk: DataClock::Register,       // 0x044
    __reserved_1: [u32; 17],                   // 0x048
    pub tcon0_io_trigger: IoTrigger::Register, // 0x08C
    pub ctrl: Control::Register,               // 0x090
    pub timing_src: Timing0::Register,         // 0x094
    pub timing_scale: Timing1::Register,       // 0x098
    pub timing_out: Timing2::Register,         // 0x09C
    pub timing_h: Timing3::Register,           // 0x0A0
    pub timing_v: Timing4::Register,           // 0x0A4
    pub timing_sync: Timing5::Register,        // 0x0A8
    __reserved_2: [u32; 17],                   // 0x0AC
    pub io_polarity: IoPolarity::Register,     // 0x0F0
    pub io_trigger: IoTrigger::Register,       // 0x0F4
}

pub struct TCON1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TCON1 {}

impl TCON1 {
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

impl Deref for TCON1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for TCON1 {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
