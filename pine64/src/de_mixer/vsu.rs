//! VSU

use static_assertions::const_assert_eq;

register! {
    Control,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1_0000);

#[repr(C)]
pub struct RegisterBlock {
    pub ctl: Control::Register, // 0x00
    __reserved_0: [u32; 16383], // 0x04
}
