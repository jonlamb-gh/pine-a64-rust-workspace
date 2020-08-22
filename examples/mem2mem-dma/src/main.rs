#![no_std]
#![no_main]

extern crate pine64_hal as hal;

use crate::hal::ccu::Clocks;
use crate::hal::console_writeln;
use crate::hal::pac::ccu::CCU;
use crate::hal::pac::dma::DMA;
use crate::hal::pac::pio::PIO;
use crate::hal::pac::uart0::UART0;
use crate::hal::pac::uart_common::NotConfigured;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use core::fmt::Write;
use core::pin::Pin;

// TODO
//use crate::hal::dma::{Descriptor, Dma};
use crate::hal::dma::*;

fn kernel_entry() -> ! {
    let clocks = Clocks::read();

    let ccu = unsafe { CCU::from_paddr() };
    let mut ccu = ccu.constrain();

    let pio = unsafe { PIO::from_paddr() };
    let gpio = pio.split(&mut ccu);

    let dma = unsafe { DMA::from_paddr() };

    let tx = gpio.pb.pb8.into_alternate_af2();
    let rx = gpio.pb.pb9.into_alternate_af2();

    let uart0: UART0<NotConfigured> = unsafe { UART0::from_paddr() };
    let serial = Serial::uart0(uart0, (tx, rx), 115_200.bps(), clocks, &mut ccu);
    let (mut serial, _rx) = serial.split();

    console_writeln!(serial, "mem2mem DMA example");
    console_writeln!(serial, "{:#?}", clocks);

    // TODO - this breaks
    let mut dma = dma.split(&mut ccu).ch0;

    static SRC_BUFFER: [u32; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    static mut DST_BUFFER: [u32; 8] = [0; 8];

    console_writeln!(serial, "SRC_BUFFER at 0x{:X}", SRC_BUFFER.as_ptr() as usize);
    unsafe {
        console_writeln!(serial, "DST_BUFFER at 0x{:X}", DST_BUFFER.as_ptr() as usize);
    }

    let src = Pin::new(&SRC_BUFFER);
    let dst = unsafe { Pin::new(&mut DST_BUFFER) };

    static mut DESC: Descriptor = Descriptor::new();
    unsafe {
        console_writeln!(serial, "DESC at 0x{:X}", &DESC as *const _ as usize);
    }
    let desc = unsafe { Pin::new(&mut DESC) };

    let res = TransferResources::mem_to_mem(desc, src, dst);

    console_writeln!(serial, "{:#?}", res);

    let txfr = Transfer::new(res, &mut dma);

    console_writeln!(serial, "Starting the transfer now");

    let txfr = txfr.start(&mut dma);

    // TODO - enable_interrupts()

    let res = txfr.wait(&mut dma);

    console_writeln!(serial, "Transfer complete");

    console_writeln!(serial, "{:#?}", res);

    unsafe {
        assert_eq!(SRC_BUFFER, DST_BUFFER);
    }

    loop {
        hal::cortex_a::asm::nop();
    }
}

pine64_boot::entry!(kernel_entry);
