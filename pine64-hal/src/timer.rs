//! Timers

use crate::ccu::{Ccu, Clocks};
use crate::hal::timer::{Cancel, CountDown, Periodic};
use crate::pac::ccu::{BusClockGating0, BusSoftReset0};
use crate::pac::hstimer::{self, HSTIMER};
use crate::pac::timer::{
    Control, IrqEnable, IrqStatus, RegisterBlock as TimerRegisterBlock, TIMER,
};
use core::convert::Infallible;
use core::ops::{Deref, DerefMut};
use cortex_a::asm;
use embedded_time::rate::Hertz;
use void::Void;

// TODO
// - macro impl for timer 0/1
// - prescale config

pub trait TimerExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

pub struct Parts {
    pub tim0: TIM0,
    //pub tim1: TIM1,
}

pub struct TIM0 {
    _tim: (),
}

pub struct TIM1 {
    _tim: (),
}

impl TimerExt for TIMER {
    type Parts = Parts;

    fn split(self) -> Self::Parts {
        Parts {
            tim0: TIM0 { _tim: () },
        }
    }
}

/// Interrupt events
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClockSource {
    Osc24M,
    Osc32K,
}

impl ClockSource {
    fn frequency(self) -> Hertz {
        match self {
            ClockSource::Osc24M => Clocks::OSC_24M_FREQ,
            ClockSource::Osc32K => Clocks::OSC_32K_FREQ,
        }
    }
}

/// Hardware timer
pub struct Timer<TIM> {
    tim: TIM,
    clock_src: ClockSource,
    clock: Hertz,
    timeout: Hertz,
}

impl Timer<TIM0> {
    pub fn timer0(tim: TIM0, clock: ClockSource) -> Self {
        // TIMER doesn't have reset or gating CCU registers

        let mut timer = Timer {
            tim,
            clock_src: clock,
            clock: clock.frequency(),
            timeout: Hertz(0),
        };

        timer.disable();
        timer.unlisten(Event::TimeOut);

        timer
    }

    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => {
                self.tim
                    .irq_enable
                    .modify(IrqEnable::Timer0IrqEnable::Clear);
            }
        }
    }

    fn enable(&mut self) {
        self.tim.ctrl0.modify(Control::Enable::Set);
    }

    fn disable(&mut self) {
        self.tim.ctrl0.modify(Control::Enable::Clear);
    }
}

impl Periodic for Timer<TIM0> {}

impl CountDown for Timer<TIM0> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        self.disable();

        self.timeout = timeout.into();
        let ticks = self.clock.0 / self.timeout.0;

        let clock_src = match self.clock_src {
            ClockSource::Osc24M => Control::ClockSrc::Clock24M,
            ClockSource::Osc32K => Control::ClockSrc::Clock32K,
        };

        self.tim.intv0.write(ticks);
        self.tim
            .ctrl0
            .modify(Control::Mode::Continuous + Control::Prescale::Div1 + clock_src);
        self.tim.ctrl0.modify(Control::Reload::Set);

        while self.tim.ctrl0.is_set(Control::Reload::Set) {
            asm::nop();
        }

        self.enable();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if !self
            .tim
            .irq_status
            .is_set(IrqStatus::Timer0IrqPending::Read)
        {
            Err(nb::Error::WouldBlock)
        } else {
            self.tim.irq_status.modify(IrqStatus::Timer0IrqPending::Set);
            Ok(())
        }
    }
}

impl Cancel for Timer<TIM0> {
    type Error = Infallible;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.disable();
        Ok(())
    }
}

impl Timer<HSTIMER> {
    pub fn hstimer(tim: HSTIMER, clocks: Clocks, ccu: &mut Ccu) -> Self {
        ccu.bsr0.rstr().modify(BusSoftReset0::HsTimer::Clear);
        ccu.bsr0.rstr().modify(BusSoftReset0::HsTimer::Set);
        ccu.bcg0.enr().modify(BusClockGating0::HsTimer::Set);

        let mut timer = Timer {
            tim,
            clock_src: ClockSource::Osc24M, // Not used by HS timer
            clock: clocks.ahb1(),
            timeout: Hertz(0),
        };

        timer.disable();
        timer.unlisten(Event::TimeOut);

        timer
    }

    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => {
                self.tim
                    .irq_enable
                    .modify(hstimer::IrqEnable::Enable::Clear);
            }
        }
    }

    fn enable(&mut self) {
        self.tim.ctrl.modify(hstimer::Control::Enable::Set);
    }

    fn disable(&mut self) {
        self.tim.ctrl.modify(hstimer::Control::Enable::Clear);
    }
}

impl Periodic for Timer<HSTIMER> {}

impl CountDown for Timer<HSTIMER> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        self.disable();

        self.timeout = timeout.into();

        let ticks_lo = self.clock.0 / self.timeout.0;

        self.tim
            .intv_hi
            .modify(hstimer::IntervalHigh::Value::Field::new(0).unwrap());
        self.tim.intv_lo.write(ticks_lo);

        self.tim
            .ctrl
            .modify(hstimer::Control::Mode::Continuous + hstimer::Control::Prescale::Div1);
        self.tim.ctrl.modify(hstimer::Control::Reload::Set);

        self.enable();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if !self
            .tim
            .irq_status
            .is_set(hstimer::IrqStatus::IrqPending::Read)
        {
            Err(nb::Error::WouldBlock)
        } else {
            self.tim
                .irq_status
                .modify(hstimer::IrqStatus::IrqPending::Set);
            Ok(())
        }
    }
}

impl Cancel for Timer<HSTIMER> {
    type Error = Infallible;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.disable();
        Ok(())
    }
}

impl Deref for TIM0 {
    type Target = TimerRegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER::ptr() }
    }
}

impl DerefMut for TIM0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *TIMER::mut_ptr() }
    }
}
