#![no_std]
#![no_main]

extern crate pine64_lts_bsp as bsp;

use bsp::display::*;
use bsp::hal::ccu::Clocks;
use bsp::hal::console_writeln;
use bsp::hal::pac::de::DE;
use bsp::hal::pac::de_mixer::MIXER1;
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
    let mixer1 = unsafe { MIXER1::from_paddr() };
    let de = unsafe { DE::from_paddr() };

    let edid_block = [0_u8; HDMI_EDID_BLOCK_SIZE];

    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const BPP: usize = 4;
    //const BUFFER_SIZE: usize = WIDTH * HEIGHT * BPP;
    const BUFFER_SIZE: usize = 0x01FF_0000;
    const BUFFER_SIZE_U32: usize = BUFFER_SIZE / 4;
    let frame_buffer_mem = unsafe {
        static mut FRAME_BUFFER_MEM: [u32; BUFFER_SIZE_U32] = [0; BUFFER_SIZE_U32];
        &mut FRAME_BUFFER_MEM[..]
    };

    //BUFFER_SIZE 8294400 == 0x7E_9000
    //BUFFER_SIZE_U32 2073600 == 0x1F_A400
    //addr 0x4200_88F0
    //
    // 0x100000 align
    // video_reserve: Reserving 1ff_0000 bytes at be00_0000 for video device
    // 'sunxi_de2' Video frame buffers from be000000 to bfff0000

    console_writeln!(serial, "BUFFER_SIZE {} == 0x{:X}", BUFFER_SIZE, BUFFER_SIZE);
    console_writeln!(
        serial,
        "BUFFER_SIZE_U32 {} == 0x{:X}",
        BUFFER_SIZE_U32,
        BUFFER_SIZE_U32
    );
    console_writeln!(serial, "addr 0x{:X}", frame_buffer_mem.as_ptr() as usize);

    console_writeln!(serial, "Creating the display");

    let display = HdmiDisplay::new(
        serial,
        tcon1,
        mixer1,
        de,
        hdmi,
        edid_block,
        frame_buffer_mem,
        &mut ccu,
    );

    //console_writeln!(serial, "Done with display setup");

    loop {
        bsp::hal::cortex_a::asm::nop();
    }
}

pine64_boot::entry!(kernel_entry);
