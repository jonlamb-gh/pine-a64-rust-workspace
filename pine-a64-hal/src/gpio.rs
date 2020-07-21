//! General Purpose Input / Output
//!
//! Abstracted over the PIO (CPUx-PORT port controller)
//!
//! Port B (PB): 10 input/output port
//! Port C (PC): 17 input/output port
//! Port D (PD): 25 input/output port
//! Port E (PE): 18 input/output port
//! Port F (PF):  7 input/output port
//! Port G (PG): 14 input/output port
//! Port H (PH): 12 input/output port

//use crate::hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
//use crate::pac::pio::*;
//use crate::pac::ccu::CCU;
use crate::ccu::Ccu;
use crate::pac::ccu::BusClockGating2;
use crate::pac::pio::PIO;
use core::marker::PhantomData;
//use core::ops::{Deref, DerefMut};

// PIO
// PxConfigY
// - PxY_Select values 3 bits
//    * 000: input
//    * 001: output
//    * 010: AF0
//    * 011: AF1
//    * 100: AF2
//    * 101: AF3
//    * 110: AF4
//    * 111: disabled
//
// cfg0: pins 0..=7
// cfg1: pins 8..=15
// cfg2: pins 16..=23
// cfg3: pin 24 (24..=31)

pub trait GpioExt {
    type Parts;

    fn split(self, ccu: &mut Ccu) -> Self::Parts;
}

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;

/// Input mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Alternate function mode
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Disabled mode
pub struct Disabled;

// See sun50i-a64.dtsi for drive strength values
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DriveStrength {
    L0_10mA,
    L1_20mA,
    L2_30mA,
    L3_40mA,
}

pub struct Gpio {
    pub pb: gpiob::PortB,
    // pc ...
}

impl GpioExt for PIO {
    type Parts = Gpio;

    fn split(self, ccu: &mut Ccu) -> Self::Parts {
        ccu.bcg2.enr().modify(BusClockGating2::Pio::Set);
        Gpio {
            pb: gpiob::PortB::_new(),
        }
    }
}

pub mod gpiob {
    use super::{
        Alternate, Disabled, Floating, Input, Output, PullDown, PullUp, PushPull, AF0, AF1, AF2,
        AF3, AF4,
    };
    use crate::hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
    use crate::pac::pio::{Config0, Config1, Config2, Config3, Data, Pull0, Pull1, PIO};
    use core::convert::Infallible;
    use core::marker::PhantomData;

    pub struct PortB {
        pub pb0: PB0<Disabled>,
        //pub pb1: PB1<Disabled>,
    }

    impl PortB {
        pub(super) fn _new() -> Self {
            PortB {
                pb0: PB0 {
                    //pin: 0,
                    _mode: PhantomData,
                },
                // pb1..=pb9
            }
        }
    }

    // PB has pins 0..=9
    pub struct PB0<MODE> {
        //pin: u8,
        _mode: PhantomData<MODE>,
    }

    impl<MODE> OutputPin for PB0<Output<MODE>> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(unsafe { (*PIO::mut_ptr()).pb_data.modify(Data::Pin0::Set) })
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(unsafe { (*PIO::mut_ptr()).pb_data.modify(Data::Pin0::Clear) })
        }
    }

    impl<MODE> InputPin for PB0<Input<MODE>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            self.is_low().map(|b| !b)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { (*PIO::ptr()).pb_data.is_set(Data::Pin0::Read) } != true)
        }
    }

    impl<MODE> StatefulOutputPin for PB0<Output<MODE>> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            self.is_set_low().map(|b| !b)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { (*PIO::ptr()).pb_data.is_set(Data::Pin0::Read) } != true)
        }
    }

    impl<MODE> toggleable::Default for PB0<Output<MODE>> {}

    impl<MODE> PB0<MODE> {
        #[inline]
        pub fn into_floating_input(self) -> PB0<Input<Floating>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Input) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_pull_down_input(self) -> PB0<Input<PullDown>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Input) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::PullDown) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_pull_up_input(self) -> PB0<Input<PullUp>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Input) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::PullUp) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_push_pull_output(self) -> PB0<Output<PushPull>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Output) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_alternate_af0(self) -> PB0<Alternate<AF0>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Af0) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_alternate_af1(self) -> PB0<Alternate<AF1>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Af1) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_alternate_af2(self) -> PB0<Alternate<AF2>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Af2) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_alternate_af3(self) -> PB0<Alternate<AF3>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Af3) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }

        #[inline]
        pub fn into_alternate_af4(self) -> PB0<Alternate<AF4>> {
            unsafe { (*PIO::mut_ptr()).pb_cfg0.modify(Config0::Pin0::Af4) };
            unsafe { (*PIO::mut_ptr()).pb_pull0.modify(Pull0::Pin0::Disabled) };
            PB0 { _mode: PhantomData }
        }
    }
}
