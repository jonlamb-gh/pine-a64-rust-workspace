//! Delay

use crate::hal::blocking::delay::{DelayMs, DelayUs};
use crate::hal::timer::{Cancel, CountDown};
use crate::pac::hstimer::HSTIMER;
use crate::{
    ccu::{Ccu, Clocks},
    timer::Timer,
};
use cortex_a::asm;
use embedded_time::{
    duration::{Duration, Microseconds, Milliseconds},
    rate::Hertz,
};
use nb::block;

/// High-speed timer (HSTIMER) as a delay provider
pub struct Delay {
    timer: Timer<HSTIMER>,
}

impl Delay {
    pub fn new(timer: HSTIMER, clocks: Clocks, ccu: &mut Ccu) -> Self {
        Delay {
            timer: Timer::hstimer(timer, clocks, ccu),
        }
    }
}

impl DelayUs<Microseconds> for Delay {
    fn delay_us(&mut self, us: Microseconds) {
        self.timer.start(us.to_rate::<Hertz>().unwrap());
        block!(self.timer.wait()).unwrap();
        self.timer.cancel().unwrap();
    }
}

impl DelayMs<Milliseconds> for Delay {
    fn delay_ms(&mut self, ms: Milliseconds) {
        self.timer.start(ms.to_rate::<Hertz>().unwrap());
        block!(self.timer.wait()).unwrap();
        self.timer.cancel().unwrap();
    }
}

// TODO - rm these or make a NoOpDelay type that uses Clocks...
pub(crate) fn delay_ms(ms: usize) {
    delay_us(ms * 1000);
}

pub(crate) fn delay_us(us: usize) {
    for _ in 0..us {
        for _ in 0..(24 + 10) {
            asm::nop();
        }
    }
}
