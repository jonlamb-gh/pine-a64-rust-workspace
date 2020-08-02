//! UI channel

use super::NUM_CHANNEL_CONFIGS;
use static_assertions::const_assert_eq;

// See https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/sun4i/sun8i_mixer.h#L75
// for the format values
register! {
    Attr,
    u32,
    RW,
    Fields [
         Enable WIDTH(U1) OFFSET(U0),
         Format WIDTH(U4) OFFSET(U8) [
             XRgb8888 = U4,
             Rgb565 = U10
         ]
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

const_assert_eq!(core::mem::size_of::<ConfigRegisterSubBlock>(), 0x020);

#[repr(C)]
pub struct ConfigRegisterSubBlock {
    pub attr: Attr::Register,          // 0x000
    pub size: Size::Register,          // 0x004
    pub coord: Coord::Register,        // 0x008
    pub pitch: Pitch::Register,        // 0x00C
    pub top_laddr: TopLAddr::Register, // 0x010
    pub bot_laddr: BotLAddr::Register, // 0x014
    pub fcolor: FColor::Register,      // 0x018
    __reserved_0: u32,                 // 0x01C
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1000);

#[repr(C)]
pub struct RegisterBlock {
    pub cfg: [ConfigRegisterSubBlock; NUM_CHANNEL_CONFIGS], // 0x000
    pub top_haddr: TopHAddr::Register,                      // 0x080
    pub bot_haddr: BotHAddr::Register,                      // 0x084
    pub ovl_size: OvlSize::Register,                        // 0x088
    __reserved_0: [u32; 989],                               // 0x08C
}
