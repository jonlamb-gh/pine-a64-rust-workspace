#![no_std]
#![no_main]

extern crate pine64_hal as hal;

use core::fmt::Write;
use hal::ccu::Clocks;
use hal::console_writeln;
use hal::delay::Delay;
use hal::pac::{ccu::CCU, hstimer::HSTIMER, pio::PIO, uart0::UART0, uart_common::NotConfigured};
use hal::prelude::*;
use hal::serial::Serial;

fn kernel_entry() -> ! {
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);

    let hs_timer = unsafe { HSTIMER::from_paddr() };

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), 115_200.bps(), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();

    console_writeln!(serial, "High-speed timer delay example");

    console_writeln!(serial, "{:#?}", clocks);

    let mut delay = Delay::new(hs_timer, clocks, &mut ccu);

    let mut cntr: usize = 0;
    loop {
        delay.delay_ms(1000_u32.milliseconds());
        console_writeln!(serial, "Timer expired {}", cntr);
        cntr = cntr.overflowing_add(1).0;
    }
}

pine64_boot::entry!(kernel_entry);
