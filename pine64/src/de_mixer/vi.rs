//! VI channel

use super::NUM_CHANNEL_CONFIGS;
use static_assertions::const_assert_eq;

register! {
    Attr,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Size,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Coord,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Pitch,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    TopLAddr,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    BotLAddr,
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
    TopHAddr,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    BotHAddr,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    OvlSize,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Horiz,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Vert,
    u32,
    RW,
    Fields [
         Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<ConfigRegisterSubBlock>(), 0x030);

#[repr(C)]
pub struct ConfigRegisterSubBlock {
    pub attr: Attr::Register,               // 0x000
    pub size: Size::Register,               // 0x004
    pub coord: Coord::Register,             // 0x008
    pub pitch: [Pitch::Register; 3],        // 0x00C
    pub top_laddr: [TopLAddr::Register; 3], // 0x018
    pub bot_laddr: [BotLAddr::Register; 3], // 0x024
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1000);

#[repr(C)]
pub struct RegisterBlock {
    pub cfg: [ConfigRegisterSubBlock; NUM_CHANNEL_CONFIGS], // 0x000
    pub fcolor: [FColor::Register; NUM_CHANNEL_CONFIGS],    // 0x0C0
    pub top_haddr: [TopHAddr::Register; 3],                 // 0x0D0
    pub bot_haddr: [BotHAddr::Register; 3],                 // 0x0DC
    pub ovl_size: [OvlSize::Register; 2],                   // 0x0E8
    pub horiz: [Horiz::Register; 2],                        // 0x0F0
    pub vert: [Vert::Register; 2],                          // 0x0F8
    __reserved_0: [u32; 960],                               // 0x0FC
}
