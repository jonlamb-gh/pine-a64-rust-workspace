//! TCON0
//!
//! Size: 4K

//use core::marker::PhantomData;
//use core::ops::{Deref, DerefMut};
use static_assertions::const_assert_eq;

pub const PADDR: usize = 0x01C0_C000;

register! {
    GlobalControl,
    u32,
    RW,
    Fields [
        IoMapSelect WIDTH(U1) OFFSET(U0),
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
    FrmControl,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    FrmSeed,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    FrmTable,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    Fifo3d,
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
    DataClock,
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
    HvIface,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuIface,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuWriteData,
    u32,
    WO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuReadData0,
    u32,
    RO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    CpuReadData1,
    u32,
    RO,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

register! {
    LvdsIface,
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

register! {
    LvdsAnalog0,
    u32,
    RW,
    Fields [
        Bits WIDTH(U32) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x224);

#[repr(C)]
pub struct RegisterBlock {
    pub gctrl: GlobalControl::Register,      // 0x000
    pub gint0: GlobalInt0::Register,         // 0x004
    pub gint1: GlobalInt1::Register,         // 0x008
    __reserved_0: u32,                       // 0x00C
    pub frm_ctrl: FrmControl::Register,      // 0x010
    pub frm_seed: [FrmSeed::Register; 6],    // 0x014
    pub frm_table: [FrmTable::Register; 4],  // 0x02C
    pub fifo_3d: Fifo3d::Register,           // 0x03C
    pub ctrl: Control::Register,             // 0x040
    pub dclk: DataClock::Register,           // 0x044
    pub timing_active: Timing0::Register,    // 0x048
    pub timing_h: Timing1::Register,         // 0x04C
    pub timing_v: Timing2::Register,         // 0x050
    pub timing_sync: Timing3::Register,      // 0x054
    pub hv_iface: HvIface::Register,         // 0x058
    __reserved_1: u32,                       // 0x05C
    pub cpu_iface: CpuIface::Register,       // 0x060
    pub cpu_wdata: CpuWriteData::Register,   // 0x064
    pub cpu_rdata0: CpuReadData0::Register,  // 0x068
    pub cpu_rdata1: CpuReadData1::Register,  // 0x06C
    __reserved_2: [u32; 5],                  // 0x070
    pub lvds_iface: LvdsIface::Register,     // 0x084
    pub io_polarity: IoPolarity::Register,   // 0x088
    pub io_trigger: IoTrigger::Register,     // 0x08C
    __reserved_3: [u32; 100],                // 0x090
    pub lvds_analog0: LvdsAnalog0::Register, // 0x220
}
