#![no_std]
//#![deny(warnings)]

use embedded_hal as hal;

pub use cortex_a;
pub use pine64 as pac;

pub mod ccu;
pub mod dma;
pub mod gpio;
pub mod prelude;
pub mod serial;
pub mod units;

pub mod state {
    /// Indicates that a peripheral is enabled
    pub struct Enabled;

    /// Indicates that a peripheral is disabled
    pub struct Disabled;
}
