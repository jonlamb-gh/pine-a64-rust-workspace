use crate::ccu::{Ccu, Clocks};
use crate::gpio::{
    Alternate, AF0, AF1, AF2, PB0, PB1, PB2, PB3, PB8, PB9, PD0, PD1, PD2, PD3, PD4, PD5,
};
use crate::hal::serial;
use crate::pac::ccu::{BusClockGating3, BusSoftReset4};
use crate::pac::uart_common::{
    DivisorLatchHigh, DivisorLatchLow, FifoControl, IntEnable, LineControl, LineStatus,
    NotConfigured, Receive, ReceiveHolding, Status, Transmit, TransmitHolding,
};
use crate::pac::{uart0::UART0, uart1::UART1, uart2::UART2, uart3::UART3, uart4::UART4};
use core::convert::Infallible;
use core::fmt;
use core::marker::PhantomData;
use embedded_time::rate::BitsPerSecond;
use nb::block;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    /// Framing error
    Framing,
    /// Rx FIFO error
    Fifo,
}

/// Alias to `write!` that drops the result
#[macro_export]
macro_rules! console_write {
    ($dst:expr, $($arg:tt)*) => (write!($dst, $($arg)*).ok())
}

/// Alias to `writeln!` that drops the result
#[macro_export]
macro_rules! console_writeln {
    ($dst:expr) => (
        write!($dst, "\n").ok()
    );
    ($dst:expr,) => (
        writeln!($dst).ok()
    );
    ($dst:expr, $($arg:tt)*) => (
        writeln!($dst, $($arg)*).ok()
    );
}

// TODO - these should be "closed" traits
pub trait Pins<UART> {}
pub trait PinTx<UART> {}
pub trait PinRx<UART> {}
pub trait PinRts<UART> {}
pub trait PinCts<UART> {}

impl<UART, TX, RX> Pins<UART> for (TX, RX)
where
    TX: PinTx<UART>,
    RX: PinRx<UART>,
{
}

impl<UART, TX, RX, RTS, CTS> Pins<UART> for (TX, RX, RTS, CTS)
where
    TX: PinTx<UART>,
    RX: PinRx<UART>,
    RTS: PinRts<UART>,
    CTS: PinCts<UART>,
{
}

impl<RxTx> PinTx<UART0<RxTx>> for PB8<Alternate<AF2>> {}
impl<RxTx> PinRx<UART0<RxTx>> for PB9<Alternate<AF2>> {}

// TODO UART1 pins

impl<RxTx> PinTx<UART2<RxTx>> for PB0<Alternate<AF0>> {}
impl<RxTx> PinRx<UART2<RxTx>> for PB1<Alternate<AF0>> {}
impl<RxTx> PinRts<UART2<RxTx>> for PB2<Alternate<AF0>> {}
impl<RxTx> PinCts<UART2<RxTx>> for PB3<Alternate<AF0>> {}

// TODO - PH pins
impl<RxTx> PinTx<UART3<RxTx>> for PD0<Alternate<AF1>> {}
impl<RxTx> PinRx<UART3<RxTx>> for PD1<Alternate<AF1>> {}

impl<RxTx> PinTx<UART4<RxTx>> for PD2<Alternate<AF1>> {}
impl<RxTx> PinRx<UART4<RxTx>> for PD3<Alternate<AF1>> {}
impl<RxTx> PinRts<UART4<RxTx>> for PD4<Alternate<AF1>> {}
impl<RxTx> PinCts<UART4<RxTx>> for PD5<Alternate<AF1>> {}

pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<UART> {
    _uart: PhantomData<UART>,
}

/// Serial transmitter
pub struct Tx<UART> {
    _uart: PhantomData<UART>,
}

macro_rules! hal {
    ($(
        $UARTX:ident: ($uartX:ident, $UartX:ident, $BCGr:ident, $BCGt:ident, $BSRr:ident, $BSRt:ident),
    )+) => {
        $(
            impl<PINS, RxTx> Serial<$UARTX<RxTx>, PINS> {
                pub fn $uartX(
                    uart: $UARTX<RxTx>,
                    pins: PINS,
                    baud_rate: BitsPerSecond,
                    clocks: Clocks,
                    ccu: &mut Ccu,
                ) -> Self
                where
                    PINS: Pins<$UARTX<RxTx>>,
                {
                    ccu.$BCGr.enr().modify($BCGt::$UartX::Set);
                    ccu.$BSRr.rstr().modify($BSRt::$UartX::Clear);
                    ccu.$BSRr.rstr().modify($BSRt::$UartX::Set);
                    let _ = ccu.$BSRr.rstr().is_set($BSRt::$UartX::Set);

                    // Disable UART
                    unsafe {
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .lcr
                            .modify(LineControl::DivisorLatchAccess::Set);
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .dll
                            .modify(DivisorLatchLow::Lsb::Clear);
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .dlh
                            .modify(DivisorLatchHigh::Msb::Clear);
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .lcr
                            .modify(LineControl::DivisorLatchAccess::Clear);
                    }

                    // Disable interrupts
                    unsafe {
                        (*$UARTX::<Transmit>::mut_ptr())
                            .ier
                            .modify(IntEnable::Erbfi::Clear + IntEnable::Etbei::Clear
                                + IntEnable::Elsi::Clear + IntEnable::Edssi::Clear
                                + IntEnable::Ptime::Clear)
                    };

                    // Disable FIFOs
                    unsafe {
                        (*$UARTX::<Transmit>::mut_ptr())
                            .fcr
                            .modify(FifoControl::FifoEnable::Clear)
                    };

                    // Clear MCR
                    unsafe { (*$UARTX::<NotConfigured>::mut_ptr()).mcr.write(0) };

                    // Setup 8-N-1
                    unsafe {
                        (*$UARTX::<NotConfigured>::mut_ptr()).lcr.modify(
                            LineControl::DataLength::EightBits
                                + LineControl::StopBits::One
                                + LineControl::ParityEnable::Clear,
                        )
                    };

                    // Enable and reset FIFOs
                    unsafe {
                        (*$UARTX::<Transmit>::mut_ptr())
                            .fcr
                            .modify(FifoControl::FifoEnable::Set)
                    };

                    // Setup baudrate, enable UART
                    //
                    // Baudrate = serial-clock / (16 * divisor)
                    // => divisor = (serial-clock / 16) / baudrate
                    //
                    // Add half of the denominator to deal with rounding errors
                    let divisor = (clocks.apb2().0 + (8 * baud_rate.0)) / (16 * baud_rate.0);
                    let lsb = divisor & 0xFF;
                    let msb = (divisor >> 8) & 0xFF;

                    unsafe {
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .lcr
                            .modify(LineControl::DivisorLatchAccess::Set);
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .dll
                            .modify(DivisorLatchLow::Lsb::Field::new(lsb).unwrap());
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .dlh
                            .modify(DivisorLatchHigh::Msb::Field::new(msb).unwrap());
                        (*$UARTX::<NotConfigured>::mut_ptr())
                            .lcr
                            .modify(LineControl::DivisorLatchAccess::Clear);
                    }

                    Serial { uart, pins }
                }

                pub fn split(self) -> (Tx<$UARTX<RxTx>>, Rx<$UARTX<RxTx>>) {
                    (Tx { _uart: PhantomData }, Rx { _uart: PhantomData })
                }

                pub fn free(self) -> ($UARTX<RxTx>, PINS) {
                    (self.uart, self.pins)
                }
            }

            impl<RxTx> serial::Read<u8> for Rx<$UARTX<RxTx>> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    let uart = unsafe { &mut *$UARTX::<Receive>::mut_ptr() };

                    if uart.lsr.is_set(LineStatus::OverrunError::Set) {
                        Err(nb::Error::Other(Error::Overrun))
                    } else if uart.lsr.is_set(LineStatus::ParityError::Set) {
                        Err(nb::Error::Other(Error::Parity))
                    } else if uart.lsr.is_set(LineStatus::FramingError::Set) {
                        Err(nb::Error::Other(Error::Framing))
                    } else if uart.lsr.is_set(LineStatus::RxFifoError::Set) {
                        Err(nb::Error::Other(Error::Fifo))
                    } else if uart.sr.is_set(Status::RxFifoNotEmpty::Set) {
                        Ok(uart
                            .rhr
                            .get_field(ReceiveHolding::Data::Read)
                            .unwrap()
                            .val() as u8)
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }
            }

            impl<RxTx> serial::Write<u8> for Tx<$UARTX<RxTx>> {
                // TODO - any real errors?
                // FIFOs should always be enabled
                type Error = Infallible;

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    let uart = unsafe { &mut *$UARTX::<Transmit>::mut_ptr() };
                    if uart.sr.is_set(Status::TxFifoEmpty::Set) {
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    let uart = unsafe { &mut *$UARTX::<Transmit>::mut_ptr() };
                    if uart.sr.is_set(Status::TxFifoNotFull::Set) {
                        uart.thr
                            .modify(TransmitHolding::Data::Field::new(byte as _).unwrap());
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }
            }

            impl<RxTx> core::fmt::Write for Tx<$UARTX<RxTx>> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    use serial::Write;
                    for b in s.bytes() {
                        block!(self.write(b)).map_err(|_| fmt::Error)?;
                    }
                    Ok(())
                }
            }

        )+
    }
}

hal! {
    UART0: (uart0, Uart0, bcg3, BusClockGating3, bsr4, BusSoftReset4),
    UART1: (uart1, Uart1, bcg3, BusClockGating3, bsr4, BusSoftReset4),
    UART2: (uart2, Uart2, bcg3, BusClockGating3, bsr4, BusSoftReset4),
    UART3: (uart3, Uart3, bcg3, BusClockGating3, bsr4, BusSoftReset4),
    UART4: (uart4, Uart4, bcg3, BusClockGating3, bsr4, BusSoftReset4),
}
