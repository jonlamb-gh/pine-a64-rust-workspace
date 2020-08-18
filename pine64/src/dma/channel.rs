//! DMA channel

use static_assertions::const_assert_eq;

register! {
    ChannelEnable,
    u32,
    RW,
    Fields [
        Enable WIDTH(U1) OFFSET(U0),
    ]
}

register! {
    ChannelPause,
    u32,
    RW,
    Fields [
        Pause WIDTH(U1) OFFSET(U0),
    ]
}

register! {
    ChannelDescAddr,
    u32,
    RW,
    Fields [
        AddrHigh WIDTH(U1) OFFSET(U0),
        Addr WIDTH(U31) OFFSET(U1),
    ]
}

register! {
    ChannelConfig,
    u32,
    RO,
    Fields [
        SrcDrqType WIDTH(U5) OFFSET(U0),
        SrcAddrMode WIDTH(U1) OFFSET(U5) [
            Linear = U0,
            Io = U1
        ],
        SrcBlockSize WIDTH(U2) OFFSET(U6) [
            Bytes1 = U0,
            Bytes4 = U1,
            Bytes8 = U2,
            Bytes16 = U3
        ],
        SrcDataWidth WIDTH(U2) OFFSET(U9) [
            Bits8 = U0,
            Bits16 = U1,
            Bits32 = U2,
            Bits64 = U3
        ],
        DstDrqType WIDTH(U5) OFFSET(U16),
        DstAddrMode WIDTH(U1) OFFSET(U21) [
            Linear = U0,
            Io = U1
        ],
        DstBlockSize WIDTH(U2) OFFSET(U22) [
            Bytes1 = U0,
            Bytes4 = U1,
            Bytes8 = U2,
            Bytes16 = U3
        ],
        DstDataWidth WIDTH(U2) OFFSET(U25) [
            Bits8 = U0,
            Bits16 = U1,
            Bits32 = U2,
            Bits64 = U3
        ],
    ]
}

register! {
    ChannelSourceAddr,
    u32,
    RO,
    Fields [
        Addr WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    ChannelDestAddr,
    u32,
    RO,
    Fields [
        Addr WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    ChannelByteCountLeft,
    u32,
    RO,
    Fields [
        Count WIDTH(U25) OFFSET(U0),
    ]
}

register! {
    ChannelParam,
    u32,
    RO,
    Fields [
        WaitClockCycles WIDTH(U8) OFFSET(U0),
        SrcAddrHBit WIDTH(U1) OFFSET(U16),
        DstAddrHBit WIDTH(U1) OFFSET(U18),
    ]
}

register! {
    ChannelMode,
    u32,
    RW,
    Fields [
        SrcMode WIDTH(U1) OFFSET(U2) [
            Wait = U0,
            Handshake = U1
        ],
        DstMode WIDTH(U1) OFFSET(U3) [
            Wait = U0,
            Handshake = U1
        ]
    ]
}

register! {
    ChannelFormerDescAddr,
    u32,
    RO,
    Fields [
        Addr WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    ChannelPackageCounter,
    u32,
    RO,
    Fields [
        Count WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x40);

#[repr(C)]
pub struct RegisterBlock {
    pub enable: ChannelEnable::Register,                   // 0x00
    pub pause: ChannelPause::Register,                     // 0x04
    pub desc_addr: ChannelDescAddr::Register,              // 0x08
    pub config: ChannelConfig::Register,                   // 0x0C
    pub cur_src: ChannelSourceAddr::Register,              // 0x10
    pub cur_dst: ChannelDestAddr::Register,                // 0x14
    pub bcnt_left: ChannelByteCountLeft::Register,         // 0x18
    pub param: ChannelParam::Register,                     // 0x1C
    __reserved_0: [u32; 2],                                // 0x20
    pub mode: ChannelMode::Register,                       // 0x28
    pub former_desc_addr: ChannelFormerDescAddr::Register, // 0x2C
    pub pkg_cnt: ChannelPackageCounter::Register,          // 0x30
    __reserved_1: [u32; 3],                                // 0x34
}
