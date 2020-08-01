#![no_std]
#![no_main]

extern crate pine_a64_hal as hal;

//use crate::hal::prelude::*;
use crate::hal::ccu::*;
use crate::hal::gpio::*;
use crate::hal::pac;
use crate::hal::pac::ccu::CCU;
use crate::hal::pac::pio::PIO;
use crate::hal::pac::uart0::UART0;
use crate::hal::pac::uart_common::NotConfigured;
use crate::hal::serial::*;
use crate::hal::units::Bps;
use core::fmt::Write;

fn kernel_entry() -> ! {
    // TODO - need to update clocks
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), Bps(115200), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();
    writeln!(serial, "{:#?}", clocks).ok();

    loop {
        writeln!(serial, "UART0 example").ok();
    }
}

pine_a64_boot::entry!(kernel_entry);
