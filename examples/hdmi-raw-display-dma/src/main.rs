#![no_std]
#![no_main]

extern crate pine64_hal as hal;

use crate::hal::dma::{Descriptor, Transfer, TransferResources};
use crate::hal::pac::dma::DMA;
use core::fmt::Write;
use core::pin::Pin;
use hal::ccu::Clocks;
use hal::console_writeln;
use hal::display::hdmi::HdmiDisplay;
use hal::pac::{
    ccu::CCU, de::DE, de_mixer::MIXER1, hdmi::HDMI, pio::PIO, tcon1::TCON1, uart0::UART0,
    uart_common::NotConfigured,
};
use hal::prelude::*;
use hal::serial::Serial;

fn kernel_entry() -> ! {
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);
    let dma = unsafe { DMA::from_paddr() };
    let hdmi = unsafe { HDMI::from_paddr() };
    let tcon1 = unsafe { TCON1::from_paddr() };
    let mixer1 = unsafe { MIXER1::from_paddr() };
    let de = unsafe { DE::from_paddr() };

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), 115_200.bps(), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();

    console_writeln!(serial, "HDMI raw display + DMA example");
    console_writeln!(serial, "{:#?}", clocks);

    // TODO - use display info to set this up...
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const BPP: usize = 4;
    const BUFFER_SIZE: usize = WIDTH * HEIGHT * BPP;
    const BUFFER_SIZE_U32: usize = BUFFER_SIZE / 4;

    static mut BACK_BUFFER_MEM: [u32; BUFFER_SIZE_U32] = [0; BUFFER_SIZE_U32];
    static mut FRAME_BUFFER_MEM: [u32; BUFFER_SIZE_U32] = [0; BUFFER_SIZE_U32];

    let mut back_buffer_mem = unsafe { Pin::new(&mut BACK_BUFFER_MEM[..]) };
    let mut frame_buffer_mem = unsafe { Pin::new(&mut FRAME_BUFFER_MEM[..]) };

    console_writeln!(serial, "BUFFER_SIZE {} == 0x{:X}", BUFFER_SIZE, BUFFER_SIZE);
    console_writeln!(
        serial,
        "BUFFER_SIZE_U32 {} == 0x{:X}",
        BUFFER_SIZE_U32,
        BUFFER_SIZE_U32
    );
    console_writeln!(
        serial,
        "Back buffer addr 0x{:X}",
        back_buffer_mem.as_ptr() as usize
    );
    console_writeln!(
        serial,
        "Frame buffer addr 0x{:X}",
        frame_buffer_mem.as_ptr() as usize
    );

    console_writeln!(serial, "Creating the display");

    let display = HdmiDisplay::new(tcon1, mixer1, de, hdmi, &frame_buffer_mem, &mut ccu);

    console_writeln!(&mut serial, "EDID: {:#?}", display.edid());

    console_writeln!(&mut serial, "Timing: {:#?}", display.timing());

    let mut dma = dma.split(&mut ccu).ch0;
    static mut DESC: Descriptor = Descriptor::new();
    let mut desc = unsafe { Pin::new(&mut DESC) };

    const RED: u32 = 0xFF_FF_00_00;
    const GREEN: u32 = 0xFF_00_FF_00;
    const BLUE: u32 = 0xFF_00_00_FF;

    loop {
        for color in &[RED, GREEN, BLUE] {
            for pixel in back_buffer_mem.as_mut().iter_mut() {
                *pixel = *color;
            }

            let res = TransferResources::mem_to_mem(desc, back_buffer_mem, frame_buffer_mem);
            let txfr = Transfer::new(res, &mut dma);
            let txfr = txfr.start(&mut dma);
            let res = txfr.wait(&mut dma);

            let (free_desc, free_back_buffer_mem, free_frame_buffer_mem) = res.free();
            desc = free_desc;
            back_buffer_mem = free_back_buffer_mem;
            frame_buffer_mem = free_frame_buffer_mem;
        }
    }
}

pine64_boot::entry!(kernel_entry);
