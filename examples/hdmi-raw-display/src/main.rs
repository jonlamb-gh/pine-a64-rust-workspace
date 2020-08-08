#![no_std]
#![no_main]

extern crate pine64_lts_bsp as bsp;

use bsp::display::HdmiDisplay;
use bsp::hal::ccu::Clocks;
use bsp::hal::console_writeln;
use bsp::hal::pac::uart0::UART0;
use bsp::hal::pac::uart_common::NotConfigured;
use bsp::hal::pac::{ccu::CCU, hdmi::HDMI, pio::PIO, tcon1::TCON1};
use bsp::hal::prelude::*;
use bsp::hal::serial::Serial;
use core::fmt::Write;

fn kernel_entry() -> ! {
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), 115_200.bps(), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();

    console_writeln!(serial, "HDMI raw display example");
    console_writeln!(serial, "{:#?}", clocks);

    let hdmi = unsafe { HDMI::from_paddr() };
    let tcon1 = unsafe { TCON1::from_paddr() };

    // TODO
    let mut frame_buffer = [0, 0, 0, 0];

    console_writeln!(serial, "Creating the display");

    let display = HdmiDisplay::new(tcon1, hdmi, &mut frame_buffer, &mut ccu);

    console_writeln!(serial, "Done with display setup");

    loop {
        bsp::hal::cortex_a::asm::nop();
    }
}

pine64_boot::entry!(kernel_entry);
