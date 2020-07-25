//! Global control

use static_assertions::const_assert_eq;

register! {
    GlobalControl,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0),
    ]
}

register! {
    GlobalStatus,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    GlobalDBuff,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0),
    ]
}

register! {
    GlobalSize,
    u32,
    RW,
    Fields [
         SizeWidth WIDTH(U16) OFFSET(U0),
         SizeHeight WIDTH(U16) OFFSET(U16),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1000);

/// Global control
#[repr(C)]
pub struct RegisterBlock {
    pub ctrl: GlobalControl::Register,  // 0x000
    pub status: GlobalStatus::Register, // 0x004
    pub dbuf: GlobalDBuff::Register,    // 0x008
    pub size: GlobalSize::Register,     // 0x00C
    __reserved_0: [u32; 1020],          // 0x010
}
