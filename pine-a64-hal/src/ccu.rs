use crate::pac::ccu::{BusClockGating2, BusClockGating3, BusSoftReset4, CCU};
use core::convert::TryInto;
use embedded_time::{units::Hertz, Period};

pub trait CcuExt {
    fn constrain(self) -> Ccu;
}

impl CcuExt for CCU {
    fn constrain(self) -> Ccu {
        Ccu {
            bcg2: BCG2 { _0: () },
            bcg3: BCG3 { _0: () },
            bsr4: BSR4 { _0: () },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clocks {
    cpu: Hertz,
    ahb1: Hertz,
    ahb2: Hertz,
    apb1: Hertz,
    apb2: Hertz,
}

impl Clocks {
    pub fn read() -> Self {
        Clocks {
            // TODO
            // 272_000_000
            //
            // COUNTER_FREQUENCY: 24_000_000
            // uart needs apb1
            // double check device tree, might be apb2
            //
            // Clock: apb2, parent: osc24M(1), freq: 24000000
            // Clock: apb1, parent: ahb1(0), freq: 75000000
            // Clock: apb, parent: cpux(0), freq: 272000000
            // Clock: ahb0, parent: ar100(0), freq: 32768
            // awg0: AHB frequency 150000000 Hz, MDC div: 0x2
            //
            // Clock: ahb1, parent: pll_periph0(3), freq: 300000000
            // Clock: ahb2, parent: pll_periph0(1), freq: 150000000
            // Clock: cpux, parent: pll_cpux(2), freq: 816000000
            //
            // Clock: bus-uart0, parent: apb2(0), freq: 24000000
            cpu: Period::new(1, 1_000_000).try_into().expect("TODO"),
            ahb1: Period::new(1, 1_000_000).try_into().expect("TODO"),
            ahb2: Period::new(1, 1_000_000).try_into().expect("TODO"),
            apb1: Period::new(1, 24_000_000).try_into().expect("TODO"),
            apb2: Period::new(1, 1_000_000).try_into().expect("TODO"),
        }
    }

    pub fn cpu(&self) -> Hertz {
        self.cpu
    }

    pub fn ahb1(&self) -> Hertz {
        self.ahb1
    }

    pub fn ahb2(&self) -> Hertz {
        self.ahb2
    }

    pub fn apb1(&self) -> Hertz {
        self.apb1
    }

    pub fn apb2(&self) -> Hertz {
        self.apb2
    }
}

pub struct Ccu {
    pub bcg2: BCG2,
    pub bcg3: BCG3,
    pub bsr4: BSR4,
}

pub struct BCG2 {
    _0: (),
}

impl BCG2 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating2::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg2 }
    }
}

pub struct BCG3 {
    _0: (),
}

impl BCG3 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating3::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg3 }
    }
}

pub struct BSR4 {
    _0: (),
}

impl BSR4 {
    pub(crate) fn rstr(&mut self) -> &mut BusSoftReset4::Register {
        unsafe { &mut (*CCU::mut_ptr()).bsr4 }
    }
}
