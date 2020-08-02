#![no_std]
#![no_main]

extern crate pine_a64_hal as hal;

use crate::hal::ccu::Clocks;
use crate::hal::console_writeln;
use crate::hal::pac::ccu::CCU;
use crate::hal::pac::pio::PIO;
use crate::hal::pac::uart0::UART0;
use crate::hal::pac::uart_common::NotConfigured;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
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
    console_writeln!(serial, "{:#?}", clocks);

    let mut cnt: usize = 0;
    loop {
        console_writeln!(serial, "UART0 example: {}", cnt);
        for _i in 0..core::u32::MAX / 32 {
            hal::cortex_a::asm::nop();
        }
        cnt = cnt.wrapping_add(1);
    }
}

pine_a64_boot::entry!(kernel_entry);
