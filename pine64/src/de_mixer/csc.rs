//! CSC

use static_assertions::const_assert_eq;

register! {
    Control,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Coef,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x40);

#[repr(C)]
pub struct RegisterBlock {
    pub ctl: Control::Register, // 0x00
    __reserved_0: [u32; 3],     // 0x04
    pub coef11: Coef::Register, // 0x10
    pub coef12: Coef::Register, // 0x14
    pub coef13: Coef::Register, // 0x18
    pub coef14: Coef::Register, // 0x1C
    pub coef21: Coef::Register, // 0x20
    pub coef22: Coef::Register, // 0x24
    pub coef23: Coef::Register, // 0x28
    pub coef24: Coef::Register, // 0x2C
    pub coef31: Coef::Register, // 0x30
    pub coef32: Coef::Register, // 0x34
    pub coef33: Coef::Register, // 0x38
    pub coef34: Coef::Register, // 0x3C
}
