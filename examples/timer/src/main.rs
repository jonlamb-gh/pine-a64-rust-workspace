#![no_std]
#![no_main]

extern crate pine64_hal as hal;

use core::fmt::Write;
use hal::ccu::Clocks;
use hal::console_writeln;
use hal::pac::{ccu::CCU, pio::PIO, timer::TIMER, uart0::UART0, uart_common::NotConfigured};
use hal::prelude::*;
use hal::serial::Serial;
use hal::time::rate::Hertz;
use hal::timer::{ClockSource, Timer};

fn kernel_entry() -> ! {
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);

    let tims = unsafe { TIMER::from_paddr() };
    let timers = tims.split();
    let tim0 = timers.tim0;

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), 115_200.bps(), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();

    console_writeln!(serial, "Timer0 example");

    console_writeln!(serial, "{:#?}", clocks);

    let mut timer = Timer::tim0(tim0, Hertz(1), ClockSource::Osc24M);

    loop {
        if timer.wait().is_ok() {
            console_writeln!(serial, "Timer expired");
        }
    }
}

pine64_boot::entry!(kernel_entry);
