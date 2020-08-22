#![no_std]
#![no_main]

extern crate pine64_hal as hal;

use core::fmt::Write;
use core::pin::Pin;
use hal::ccu::Clocks;
use hal::console_writeln;
use hal::display::hdmi::HdmiDisplay;
use hal::pac::de::DE;
use hal::pac::de_mixer::MIXER1;
use hal::pac::uart0::UART0;
use hal::pac::uart_common::NotConfigured;
use hal::pac::{ccu::CCU, hdmi::HDMI, pio::PIO, tcon1::TCON1};
use hal::prelude::*;
use hal::serial::Serial;

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
    let mixer1 = unsafe { MIXER1::from_paddr() };
    let de = unsafe { DE::from_paddr() };

    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const BPP: usize = 4;
    const BUFFER_SIZE: usize = WIDTH * HEIGHT * BPP;

    // TODO - does the fb need to be aligned to this?
    // doesn't appear to need it
    //
    //const ALIGN: usize = 0x10_0000;
    //const BUFFER_SIZE: usize = 0x01FF_0000;

    const BUFFER_SIZE_U32: usize = BUFFER_SIZE / 4;
    static mut FRAME_BUFFER_MEM: [u32; BUFFER_SIZE_U32] = [0; BUFFER_SIZE_U32];

    let mut frame_buffer_mem = unsafe { Pin::new(&mut FRAME_BUFFER_MEM[..]) };

    console_writeln!(serial, "BUFFER_SIZE {} == 0x{:X}", BUFFER_SIZE, BUFFER_SIZE);
    console_writeln!(
        serial,
        "BUFFER_SIZE_U32 {} == 0x{:X}",
        BUFFER_SIZE_U32,
        BUFFER_SIZE_U32
    );
    console_writeln!(serial, "addr 0x{:X}", frame_buffer_mem.as_ptr() as usize);

    console_writeln!(serial, "Creating the display");

    let display = HdmiDisplay::new(tcon1, mixer1, de, hdmi, &frame_buffer_mem, &mut ccu);

    console_writeln!(&mut serial, "EDID: {:#?}", display.edid());

    console_writeln!(&mut serial, "Timing: {:#?}", display.timing());

    console_writeln!(serial, "Slowly drawing pixels");

    const RED: u32 = 0xFF_FF_00_00;
    const GREEN: u32 = 0xFF_00_FF_00;
    const BLUE: u32 = 0xFF_00_00_FF;

    loop {
        console_writeln!(serial, "Red");
        for pixel in frame_buffer_mem.iter_mut() {
            *pixel = RED;
            delay_us(100);
        }

        console_writeln!(serial, "Green");
        for pixel in frame_buffer_mem.iter_mut() {
            *pixel = GREEN;
            delay_us(100);
        }

        console_writeln!(serial, "Blue");
        for pixel in frame_buffer_mem.iter_mut() {
            *pixel = BLUE;
            delay_us(100);
        }
    }
}

fn delay_us(us: usize) {
    for _ in 0..us {
        for _ in 0..(24 + 10) {
            hal::cortex_a::asm::nop();
        }
    }
}

pine64_boot::entry!(kernel_entry);
