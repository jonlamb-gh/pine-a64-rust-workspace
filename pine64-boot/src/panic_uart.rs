use core::fmt::Write;
use core::intrinsics;
use core::panic::PanicInfo;
use pine64_hal::ccu::Clocks;
use pine64_hal::pac::ccu::CCU;
use pine64_hal::pac::pio::PIO;
use pine64_hal::pac::uart0::UART0;
use pine64_hal::pac::uart_common::NotConfigured;
use pine64_hal::prelude::*;
use pine64_hal::serial::Serial;
use pine64_hal::{console_writeln, cortex_a::asm};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    for _ in 0..1000 {
        asm::nop();
    }

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

    console_writeln!(serial, "\n\n");
    console_writeln!(serial, "{}", info);
    console_writeln!(serial, "\n\n");

    for _ in 0..1000 {
        asm::nop();
    }

    intrinsics::abort()
}
