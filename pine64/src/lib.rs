#![deny(warnings)]
#![no_std]

#[macro_use]
extern crate bounded_registers;
#[macro_use]
extern crate typenum;

pub mod ccu;
pub mod de;
pub mod de_mixer;
pub mod hdmi;
pub mod pio;
pub mod tcon0;
pub mod tcon1;
pub mod uart0;
pub mod uart1;
pub mod uart2;
pub mod uart3;
pub mod uart4;
pub mod uart_common;
