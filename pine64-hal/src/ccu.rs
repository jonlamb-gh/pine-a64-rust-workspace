use crate::pac::ccu::{
    Ahb1Apb1Config, Ahb2Config, Apb2Config, BusClockGating0, BusClockGating1, BusClockGating2,
    BusClockGating3, BusSoftReset0, BusSoftReset1, BusSoftReset4, PllCpuXControl, PllDeControl,
    PllPeriph0Control, PllVideo0Control, CCU,
};
use cortex_a::asm;
use embedded_time::rate::Hertz;

pub trait CcuExt {
    fn constrain(self) -> Ccu;
}

impl CcuExt for CCU {
    fn constrain(self) -> Ccu {
        Ccu {
            bcg0: BCG0 { _0: () },
            bcg1: BCG1 { _0: () },
            bcg2: BCG2 { _0: () },
            bcg3: BCG3 { _0: () },
            bsr0: BSR0 { _0: () },
            bsr1: BSR1 { _0: () },
            bsr4: BSR4 { _0: () },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
    pub const OSC_24M_FREQ: Hertz = Hertz(24_000_000);
    pub const OSC_32K_FREQ: Hertz = Hertz(32_768);
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
        let pll_cpu = (Self::OSC_24M_FREQ.0 * pll_cpu_n * pll_cpu_k) / (pll_cpu_m * pll_cpu_div_p);

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
        let pll_periph0_1x = Self::OSC_24M_FREQ.0 * pll_p0_n * (pll_p0_k / 2);

        // PLL_PERIPH0(2X) = 24MHz * N * K
        let pll_periph0_2x = Self::OSC_24M_FREQ.0 * pll_p0_n * pll_p0_k;

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
            pll_periph0_1x: Hertz::new(pll_periph0_1x),
            pll_periph0_2x: Hertz::new(pll_periph0_2x),
            cpu: Hertz::new(pll_cpu),
            ahb1: Hertz::new(ahb1_clk),
            ahb2: Hertz::new(ahb2_clk),
            apb1: Hertz::new(apb1_clk),
            apb2: Hertz::new(apb2_clk.0),
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
    pub bcg0: BCG0,
    pub bcg1: BCG1,
    pub bcg2: BCG2,
    pub bcg3: BCG3,
    // bsr0: AHB1 Reset 0
    // bsr1: AHB1 Reset 1
    // bsr2: AHB1 Reset 2
    // bsr3: APB1 Reset
    // bsr4: APB2 Reset
    pub bsr0: BSR0,
    pub bsr1: BSR1,
    pub bsr4: BSR4,
}

// TODO - rename the wrappers
// - BSR4 -> APB2

pub struct BCG0 {
    _0: (),
}

impl BCG0 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating0::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg0 }
    }
}

pub struct BCG1 {
    _0: (),
}

impl BCG1 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating1::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg1 }
    }
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

pub struct BSR0 {
    _0: (),
}

impl BSR0 {
    pub(crate) fn rstr(&mut self) -> &mut BusSoftReset0::Register {
        unsafe { &mut (*CCU::mut_ptr()).bsr0 }
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

impl Ccu {
    pub(crate) fn pll_video0(&self) -> Hertz {
        let ccu = unsafe { &*CCU::ptr() };

        let n = 1 + ccu
            .pll_video0
            .get_field(PllVideo0Control::FactorN::Read)
            .unwrap()
            .val();
        let m = 1 + ccu
            .pll_video0
            .get_field(PllVideo0Control::PreDivM::Read)
            .unwrap()
            .val();

        let f = (24000 * n / m) * 1000;
        Hertz::new(f)
    }

    pub(crate) fn set_pll_video0(&mut self, clk: Hertz) {
        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // 6 MHz steps to allow higher frequency for DE2
        let m = 4;

        if clk.0 == 0 {
            ccu.pll_video0.modify(PllVideo0Control::Enable::Clear);
        } else {
            let n = clk.0 / (Clocks::OSC_24M_FREQ.0 / m);
            let factor_n = n - 1;
            let factor_m = m - 1;
            // PLL3 rate = 24000000 * n / m
            ccu.pll_video0.modify(
                PllVideo0Control::Enable::Set
                    + PllVideo0Control::Mode::Integer
                    + PllVideo0Control::FactorN::Field::new(factor_n).unwrap()
                    + PllVideo0Control::PreDivM::Field::new(factor_m).unwrap(),
            );

            while !ccu.pll_video0.is_set(PllVideo0Control::Lock::Read) {
                asm::nop();
            }
        }
    }

    pub(crate) fn set_pll_video0_factors(&mut self, m: u32, n: u32) {
        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // TODO - these are trip'n ...
        //assert_ne!(m & !0xFF, 0);
        //assert_ne!(n & !0xFF, 0);

        // PLL3 rate = 24000000 * n / m
        let factor_n = n - 1;
        let factor_m = m - 1;
        ccu.pll_video0.modify(
            PllVideo0Control::Enable::Set
                + PllVideo0Control::Mode::Integer
                + PllVideo0Control::FactorN::Field::new(factor_n).unwrap()
                + PllVideo0Control::PreDivM::Field::new(factor_m).unwrap(),
        );

        while !ccu.pll_video0.is_set(PllVideo0Control::Lock::Read) {
            asm::nop();
        }
    }

    pub(crate) fn set_pll_de(&mut self, clk: Hertz) {
        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // 12 MHz steps
        let m = 2;
        if clk.0 == 0 {
            ccu.pll_de.modify(PllDeControl::Enable::Clear);
        } else {
            let n = clk.0 / (Clocks::OSC_24M_FREQ.0 / m);
            let factor_n = n - 1;
            let factor_m = m - 1;
            // PLL10 rate = 24000000 * n / m
            ccu.pll_de.modify(
                PllDeControl::Enable::Set
                    + PllDeControl::Mode::Integer
                    + PllDeControl::FactorN::Field::new(factor_n).unwrap()
                    + PllDeControl::PreDivM::Field::new(factor_m).unwrap(),
            );

            while !ccu.pll_de.is_set(PllDeControl::Lock::Read) {
                asm::nop();
            }
        }
    }
}
