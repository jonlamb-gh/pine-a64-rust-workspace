//! Display engine (DE 2.0) mixer registers
//!
//! Both mixers (MUX0 and MUX1) have the same defintions.

pub mod ase;
pub mod bld;
pub mod bws;
pub mod csc;
pub mod fcc;
pub mod fce;
pub mod global;
pub mod gsu;
pub mod lti;
pub mod mixer0;
pub mod mixer1;
pub mod peak;
pub mod ui;
pub mod vi;
pub mod vsu;

pub use mixer0::MIXER0;
pub use mixer1::MIXER1;

pub const NUM_VI_CHANNELS: usize = 1;
pub const NUM_UI_CHANNELS: usize = 3;
pub const NUM_CHANNELS: usize = NUM_VI_CHANNELS + NUM_UI_CHANNELS;
pub const NUM_CHANNEL_CONFIGS: usize = 4;

use static_assertions::const_assert_eq;

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x000B_0040);

/// Mixer/MUX registers
#[repr(C)]
pub struct RegisterBlock {
    pub global: global::RegisterBlock,            // 0x0000_0000
    pub bld: bld::RegisterBlock,                  // 0x0000_1000
    pub vi: vi::RegisterBlock,                    // 0x0000_2000
    pub ui: [ui::RegisterBlock; NUM_UI_CHANNELS], // 0x0000_3000
    __reserved_2: [u32; 26624],                   // 0x0000_6000
    pub vsu: vsu::RegisterBlock,                  // 0x0002_0000
    pub gsu1: gsu::RegisterBlock,                 // 0x0003_0000
    pub gsu2: gsu::RegisterBlock,                 // 0x0004_0000
    pub gsu3: gsu::RegisterBlock,                 // 0x0005_0000
    __reserved_3: [u32; 65536],                   // 0x0006_0000
    pub fce: fce::RegisterBlock,                  // 0x000A_0000
    pub bws: bws::RegisterBlock,                  // 0x000A_2000
    pub lti: lti::RegisterBlock,                  // 0x000A_4000
    pub peak: peak::RegisterBlock,                // 0x000A_6000
    pub ase: ase::RegisterBlock,                  // 0x000A_8000
    pub fcc: fcc::RegisterBlock,                  // 0x000A_A000
    __reserved_4: [u32; 4096],                    // 0x000A_C000
    pub csc: csc::RegisterBlock,                  // 0x000B_0000
}
