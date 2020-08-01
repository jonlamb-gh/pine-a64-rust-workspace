#![no_std]
//#![deny(warnings)]

use embedded_hal as hal;

pub use cortex_a;
pub use pine_a64 as pac;

pub mod ccu;
pub mod gpio;
pub mod prelude;
pub mod serial;
pub mod units;
