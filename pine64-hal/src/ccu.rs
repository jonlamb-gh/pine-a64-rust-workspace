use crate::pac::ccu::{
    Ahb1Apb1Config, Ahb2Config, Apb2Config, BusClockGating2, BusClockGating3, BusSoftReset1,
    BusSoftReset4, PllCpuXControl, PllPeriph0Control, CCU,
};
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
            bsr1: BSR1 { _0: () },
            bsr4: BSR4 { _0: () },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clocks {
    pll_periph0_1x: Hertz,
    pll_periph0_2x: Hertz,
    cpu: Hertz,
    ahb1: Hertz,
    ahb2: Hertz,
    apb1: Hertz,
    apb2: Hertz,
}

impl Clocks {
    pub const OSC_24M_FREQ: u32 = 24_000_000;
    //const OSC_32K_FREQ: usize = 32_768;
    //const OSC_I16M_FREQ: usize = 16_000_000;

    pub fn read() -> Self {
        // TODO - check that the locks have stabilized

        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // PLL output = (24MHz * N * K) / (M * P)
        let pll_cpu_m = 1 + ccu
            .pll_cpu_ctrl
            .get_field(PllCpuXControl::FactorM::Read)
            .unwrap()
            .val();
        let pll_cpu_k = 1 + ccu
            .pll_cpu_ctrl
            .get_field(PllCpuXControl::FactorK::Read)
            .unwrap()
            .val();
        let pll_cpu_n = 1 + ccu
            .pll_cpu_ctrl
            .get_field(PllCpuXControl::FactorN::Read)
            .unwrap()
            .val();
        let pll_cpu_div_p_field = ccu
            .pll_cpu_ctrl
            .get_field(PllCpuXControl::PllOutExtDivP::Read)
            .unwrap();
        let pll_cpu_div_p = if pll_cpu_div_p_field == PllCpuXControl::PllOutExtDivP::Divide1 {
            1
        } else if pll_cpu_div_p_field == PllCpuXControl::PllOutExtDivP::Divide2 {
            2
        } else if pll_cpu_div_p_field == PllCpuXControl::PllOutExtDivP::Divide4 {
            4
        } else {
            8
        };
        let pll_cpu = (Self::OSC_24M_FREQ * pll_cpu_n * pll_cpu_k) / (pll_cpu_m * pll_cpu_div_p);

        let pll_p0_k = 1 + ccu
            .pll_periph0
            .get_field(PllPeriph0Control::FactorK::Read)
            .unwrap()
            .val();
        let pll_p0_n = 1 + ccu
            .pll_periph0
            .get_field(PllPeriph0Control::FactorN::Read)
            .unwrap()
            .val();

        // PLL_PERIPH0(1X) = 24MHz * N * K/2
        let pll_periph0_1x = Self::OSC_24M_FREQ * pll_p0_n * (pll_p0_k / 2);

        // PLL_PERIPH0(2X) = 24MHz * N * K
        let pll_periph0_2x = Self::OSC_24M_FREQ * pll_p0_n * pll_p0_k;

        // AHB1
        let ahb1_pre_div = 1 + ccu
            .ahb1_apb1_cfg
            .get_field(Ahb1Apb1Config::Ahb1PreDiv::Read)
            .unwrap()
            .val();
        let ahb1_clk_src = ccu
            .ahb1_apb1_cfg
            .get_field(Ahb1Apb1Config::Ahb1ClockSrcSel::Read)
            .unwrap();
        let ahb1_clk = if ahb1_clk_src == Ahb1Apb1Config::Ahb1ClockSrcSel::PllPeriph01x {
            pll_periph0_1x / ahb1_pre_div
        } else {
            unimplemented!()
        };

        // AHB2
        let ahb2_clk_src = ccu
            .ahb2_cfg
            .get_field(Ahb2Config::ClockConfig::Read)
            .unwrap();
        let ahb2_clk = if ahb2_clk_src == Ahb2Config::ClockConfig::PllPeriph01xD2 {
            pll_periph0_1x / 2
        } else {
            unimplemented!()
        };

        // APB1
        let apb1_clk_ratio = ccu
            .ahb1_apb1_cfg
            .get_field(Ahb1Apb1Config::Apb1ClockDivRatio::Read)
            .unwrap();
        let apb1_clk = if apb1_clk_ratio == Ahb1Apb1Config::Apb1ClockDivRatio::Divide2 {
            ahb1_clk / 2
        } else {
            unimplemented!()
        };

        // APB2
        let apb2_clk_src = ccu
            .apb2_cfg
            .get_field(Apb2Config::ClockSrcSel::Read)
            .unwrap();
        let apb2_clk = if apb2_clk_src == Apb2Config::ClockSrcSel::Osc24M {
            Self::OSC_24M_FREQ
        } else {
            unimplemented!()
        };

        Clocks {
            pll_periph0_1x: Period::new(1, pll_periph0_1x).try_into().unwrap(),
            pll_periph0_2x: Period::new(1, pll_periph0_2x).try_into().unwrap(),
            cpu: Period::new(1, pll_cpu).try_into().unwrap(),
            ahb1: Period::new(1, ahb1_clk).try_into().unwrap(),
            ahb2: Period::new(1, ahb2_clk).try_into().unwrap(),
            apb1: Period::new(1, apb1_clk).try_into().unwrap(),
            apb2: Period::new(1, apb2_clk).try_into().unwrap(),
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
    // bsr0: AHB1 Reset 0
    // bsr1: AHB1 Reset 1
    // bsr2: AHB1 Reset 2
    // bsr3: APB1 Reset
    // bsr4: APB2 Reset
    pub bsr1: BSR1,
    pub bsr4: BSR4,
}

// TODO - rename the wrappers
// - BSR4 -> APB2

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

pub struct BSR1 {
    _0: (),
}

impl BSR1 {
    pub(crate) fn rstr(&mut self) -> &mut BusSoftReset1::Register {
        unsafe { &mut (*CCU::mut_ptr()).bsr1 }
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
