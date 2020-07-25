//! Alpha blending sub-engine

use super::NUM_CHANNELS;
use static_assertions::const_assert_eq;

register! {
    FColorControl,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    FColor,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    InSize,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Offset,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Route,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    PreMultiply,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    BkColor,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    OutputSize,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Mode,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CkControl,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CkConfig,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CkMax,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CkMin,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    OutControl,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<AttrRegisterSubBlock>(), 0x010);

#[repr(C)]
pub struct AttrRegisterSubBlock {
    pub fcolor: FColor::Register,  // 0x000
    pub in_size: InSize::Register, // 0x004
    pub offset: Offset::Register,  // 0x008
    __reserved_0: u32,             // 0x00C
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1000);

#[repr(C)]
pub struct RegisterBlock {
    pub fcolor_ctl: FColorControl::Register,        // 0x000
    pub attr: [AttrRegisterSubBlock; NUM_CHANNELS], // 0x004
    __reserved_0: [u32; 15],                        // 0x044
    pub route: Route::Register,                     // 0x080
    pub premultiply: PreMultiply::Register,         // 0x084
    pub bkcolor: BkColor::Register,                 // 0x088
    pub output_size: OutputSize::Register,          // 0x08C
    pub mode: [Mode::Register; NUM_CHANNELS],       // 0x090
    __reserved_1: [u32; 4],                         // 0x094
    pub ck_ctl: CkControl::Register,                // 0x0B0
    pub ck_cfg: CkConfig::Register,                 // 0x0B4
    __reserved_2: [u32; 2],                         // 0x0B8
    pub ck_max: [CkMax::Register; NUM_CHANNELS],    // 0x0C0
    __reserved_3: [u32; 4],                         // 0x0D0
    pub ck_min: [CkMin::Register; NUM_CHANNELS],    // 0x0E0
    __reserved_4: [u32; 4],                         // 0x0F0
    pub out_ctl: OutControl::Register,              // 0x100
    __reserved_5: [u32; 959],                       // 0x104
}
