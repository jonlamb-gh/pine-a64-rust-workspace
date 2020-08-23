//! Timers

use crate::ccu::Clocks;
use crate::hal::timer::{Cancel, CountDown, Periodic};
use crate::pac::timer::{Control, IrqEnable, IrqStatus, TIMER};
use core::convert::Infallible;
use core::marker::PhantomData;
use cortex_a::asm;
use embedded_time::rate::Hertz;
use void::Void;

// TODO
// - macro impl for timer 0/1
// - prescale config
// - high-speed timer to back the Delay impl's

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
    fn clock(self) -> Hertz {
        match self {
            ClockSource::Osc24M => Clocks::OSC_24M_FREQ,
            ClockSource::Osc32K => Clocks::OSC_32K_FREQ,
        }
    }
}

/// Hardware timer
pub struct Timer<TIM> {
    tim: TIMER,
    clock: ClockSource,
    timeout: Hertz,
    _t: PhantomData<TIM>,
}

impl Timer<TIM0> {
    pub fn tim0<T>(_tim: TIM0, timeout: T, clock: ClockSource) -> Self
    where
        T: Into<Hertz>,
    {
        // TIMER doesn't have reset or gating CCU registers

        let mut timer = Timer {
            tim: unsafe { TIMER::from_paddr() },
            clock,
            timeout: Hertz(0),
            _t: PhantomData,
        };

        timer.disable();
        timer.unlisten(Event::TimeOut);

        timer.start(timeout);

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
        let clock = self.clock.clock();
        let ticks = clock.0 / self.timeout.0;

        let clock_src = match self.clock {
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
            self.tim.irq_status.modify(IrqEnable::Timer0IrqEnable::Set);
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
