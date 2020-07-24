//! TCON1
//!
//! Size: 4K

//use core::marker::PhantomData;
//use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C0_D000;

register! {
    GlobalControl,
    u32,
    RW,
    Fields [
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
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing1,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing2,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing3,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing4,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Timing5,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
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

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x0F8);

#[repr(C)]
pub struct RegisterBlock {
    pub gctrl: GlobalControl::Register,    // 0x000
    pub gint0: GlobalInt0::Register,       // 0x004
    pub gint1: GlobalInt1::Register,       // 0x008
    __reserved_0: [u32; 33],               // 0x00C
    pub ctrl: Control::Register,           // 0x090
    pub timing_src: Timing0::Register,     // 0x094
    pub timing_scale: Timing1::Register,   // 0x098
    pub timing_out: Timing2::Register,     // 0x09C
    pub timing_h: Timing3::Register,       // 0x0A0
    pub timing_v: Timing4::Register,       // 0x0A4
    pub timing_sync: Timing5::Register,    // 0x0A8
    __reserved_1: [u32; 17],               // 0x0AC
    pub io_polarity: IoPolarity::Register, // 0x0F0
    pub io_trigger: IoTrigger::Register,   // 0x0F4
}
