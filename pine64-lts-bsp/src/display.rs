use crate::hal::ccu::{Ccu, Clocks};
use crate::hal::cortex_a::asm;
use crate::hal::pac::ccu::{
    BusClockGating1, BusSoftReset1, HdmiClockConfig, HdmiSlowClock, PllVideo0Control, CCU,
};
use crate::hal::pac::hdmi::{Control, Hpd, PhyControl, PhyPll, PhyStatus, PllDbg0, HDMI};
use crate::hal::pac::tcon1::TCON1;

// HDMI reg defs in arch/arm/include/asm/arch-sunxi/display.h
// sunxi_hdmi_reg

// arch/arm/include/asm/arch-sunxi/clock_sun6i.h
// obj-$(CONFIG_MACH_SUN50I)   += clock_sun6i.o
// CONFIG_SUNXI_DE2 = 1

// SUNXI_HDMI_BASE = 32374784 == 0x01EE_0000
// HDMI_PHY_OFFS   =    65536 == 0x0001_0000
//
// == 0x01EF_0000

// TODO
// - add Ccu abstractions to get rid of unsafe &mut *CCU::mut_ptr()
// - refactor all of the methods/functions
// - add log! debug stuff

// drivers/video/sunxi/sunxi_dw_hdmi.c
// sunxi_dw_hdmi.c: Allwinner DW HDMI bridge
// dw: DesignWare

// TODO - need a timer of sorts
// for udelay()

// sunxi_dw_hdmi_read_edid()
// just calls into dw_hdmi_read_edid()

//priv->hdmi.ioaddr = SUNXI_HDMI_BASE;
//priv->hdmi.i2c_clk_high = 0xd8;
//priv->hdmi.i2c_clk_low = 0xfe;
//priv->hdmi.reg_io_width = 1;
//priv->hdmi.phy_set = sunxi_dw_hdmi_phy_cfg;
//priv->mux = uc_plat->source_id;

// TODO - use hal-units/time
//const PHY_STATUS_TIMEOUT_US: u32 = 2000;

//const HPD_DELAY: usize = 500;
//const HDMI_PAD_CTRL0_HDP: u32 = 0xFE80_0000;
//const HDMI_PAD_CTRL1: u32 = 0x00D8_C830;
//const HDMI_PAD_CTRL1_HALVE: u32 = 1 << 6;
//const HDMI_PLL_CTRL: u32 = 0xFA4E_F708;

pub struct HdmiDisplay<'a> {
    tcon1: TCON1,
    hdmi: HDMI,
    frame_buffer: &'a mut [u32],
}

impl<'a> HdmiDisplay<'a> {
    pub fn new(tcon1: TCON1, hdmi: HDMI, frame_buffer: &'a mut [u32], ccu: &mut Ccu) -> Self {
        // TODO - checks/etc

        let mut d = HdmiDisplay {
            tcon1,
            hdmi,
            frame_buffer,
        };

        // sunxi_dw_hdmi_probe()
        d.probe(ccu);

        d
    }

    // sunxi_dw_hdmi_probe
    // drivers/video/sunxi/sunxi_dw_hdmi.c
    fn probe(&mut self, hal_provided_ccu: &mut Ccu) {
        // Set pll3 to 297 MHz
        clock_set_pll3(297_000_000, hal_provided_ccu);

        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // Set hdmi parent to pll3
        ccu.hdmi_clk_cfg
            .modify(HdmiClockConfig::ClockSel::Pll3Video0x1);

        // Set AHB gating to pass
        ccu.bsr1.modify(BusSoftReset1::Hdmi1::Clear);
        ccu.bsr1.modify(BusSoftReset1::Hdmi1::Set);
        ccu.bsr1.modify(BusSoftReset1::Hdmi0::Clear);
        ccu.bsr1.modify(BusSoftReset1::Hdmi0::Set);
        ccu.bcg1.modify(BusClockGating1::Hdmi::Set);
        ccu.hdmi_slow_clk_cfg
            .modify(HdmiSlowClock::DdcClockGating::Set);

        // Clock on
        ccu.hdmi_clk_cfg.modify(HdmiClockConfig::SClockGating::Set);

        self.phy_init();

        // TODO - error handle, can't get hpd signal
        self.wait_for_hpd().expect("TODO - Errors");

        // dw_hdmi_init(&priv->hdmi)
    }

    // sunxi_dw_hdmi_wait_for_hpd()
    fn wait_for_hpd(&mut self) -> Result<(), ()> {
        // TODO - timeout 300 us
        while !self.hdmi.phy_status.is_set(PhyStatus::PlugIn::Read) {
            delay_us(100);
        }

        return Ok(());
    }

    // sunxi_dw_hdmi_phy_init()
    fn phy_init(&mut self) {
        // HDMI PHY settings are taken as-is from Allwinner BSP code.
        // There is no documentation.
        self.hdmi.phy_ctrl.write(0);
        self.hdmi.phy_ctrl.modify(PhyControl::B0::Set);
        delay_us(5);
        self.hdmi.phy_ctrl.modify(PhyControl::B16::Set);
        self.hdmi.phy_ctrl.modify(PhyControl::B1::Set);
        delay_us(10);
        self.hdmi.phy_ctrl.modify(PhyControl::B2::Set);
        delay_us(5);
        self.hdmi.phy_ctrl.modify(PhyControl::B3::Set);
        delay_us(40);
        self.hdmi.phy_ctrl.modify(PhyControl::B19::Set);
        delay_us(100);
        self.hdmi.phy_ctrl.modify(PhyControl::B18::Set);
        self.hdmi.phy_ctrl.modify(PhyControl::B456::Full);

        // Note that Allwinner code doesn't fail in case of timeout
        // PHY_STATUS_TIMEOUT_US = 2000
        while !self.hdmi.phy_status.is_set(PhyStatus::Ready::Read) {
            asm::nop();
        }

        self.hdmi.phy_ctrl.modify(PhyControl::B891011::Full);
        self.hdmi.phy_ctrl.modify(PhyControl::B7::Set);

        self.hdmi.phy_pll.write(0x39dc5040);
        self.hdmi.phy_clk.write(0x80084343);
        delay_us(10000);
        self.hdmi.phy_unk3.write(1);
        self.hdmi.phy_pll.modify(PhyPll::B25::Set);
        delay_us(100000);
        let tmp = (self.hdmi.phy_status.read() & 0x1_F800) >> 11;
        self.hdmi
            .phy_pll
            .modify(PhyPll::B31::Set + PhyPll::B30::Set);
        self.hdmi
            .phy_pll
            .modify(PhyPll::F0::Field::new(tmp).unwrap());
        self.hdmi.phy_ctrl.write(0x01FF0F7F);
        self.hdmi.phy_unk1.write(0x80639000);
        self.hdmi.phy_unk2.write(0x0F81C405);

        // Enable read access to HDMI controller
        self.hdmi.phy_read_en.write(0x54524545);

        // Descramble register offsets
        self.hdmi.phy_unscramble.write(0x42494E47);
    }

    // sunxi_dw_hdmi_phy_cfg()
    fn phy_cfg(&mut self, mpixel_clock: u32, hal_provided_ccu: &mut Ccu) {
        let phy_div = self.pll_set(mpixel_clock / 1000, hal_provided_ccu);
        self.phy_set(mpixel_clock, phy_div);
    }

    // sunxi_dw_hdmi_pll_set()
    fn pll_set(&mut self, clk_khz: u32, hal_provided_ccu: &mut Ccu) -> u32 {
        let mut best_div = 0;
        let mut best_n = 0;
        let mut best_m = 0;
        let mut best_diff = 0x0FFFFFFF;

        for div in 1..=16 {
            let target = clk_khz * div;

            if target < 192000 {
                continue;
            }
            if target > 912000 {
                continue;
            }

            for m in 1..=16 {
                let n = (m * target) / 24000;

                if (n >= 1) && (n <= 128) {
                    let value = (24000 * n) / m / div;
                    let diff = clk_khz - value;
                    if diff < best_diff {
                        best_diff = diff;
                        best_m = m;
                        best_n = n;
                        best_div = div;
                    }
                }
            }
        }

        debug_assert_ne!(best_diff, 0);
        debug_assert_ne!(best_div, 0);
        debug_assert_ne!(best_m, 0);
        debug_assert_ne!(best_n, 0);

        clock_set_pll3_factors(best_m, best_n, hal_provided_ccu);

        best_div
    }

    // sunxi_dw_hdmi_phy_set()
    fn phy_set(&mut self, clock: u32, phy_div: u32) {
        let div = Self::get_phy_divider(clock);

        // No docs...
        match div {
            2 => {
                self.hdmi.phy_pll.write(0x39dc5040);
                self.hdmi.phy_clk.write(0x80084380 | (phy_div - 1));
                delay_ms(10);
                self.hdmi.phy_unk3.write(0x00000001);
                self.hdmi.phy_pll.modify(PhyPll::B25::Set);
                delay_ms(100);
                let tmp = (self.hdmi.phy_status.read() & 0x1_F800) >> 11;
                self.hdmi
                    .phy_pll
                    .modify(PhyPll::B31::Set + PhyPll::B30::Set);
                self.hdmi
                    .phy_pll
                    .modify(PhyPll::F0::Field::new(tmp).unwrap());
                self.hdmi.phy_ctrl.write(0x01FFFF7F);
                self.hdmi.phy_unk1.write(0x8063a800);
                self.hdmi.phy_unk2.write(0x0F81C485);
            }
            _ => unimplemented!(), // TODO - use an enum for exh match
        }
    }

    fn get_phy_divider(clock: u32) -> u32 {
        // No docs...
        if clock <= 27000000 {
            11
        } else if clock <= 74250000 {
            4
        } else if clock <= 148500000 {
            2
        } else {
            1
        }
    }

    // sunxi_dw_hdmi_enable()
    fn hdmi_enable(&mut self) {
        todo!();
    }

    // dw_hdmi_enable()
    fn dw_hdmi_enable(&mut self) {
        todo!();
    }

    // sunxi_dw_hdmi_read_edid()
    // just calls into dw_hdmi_read_edid()
    fn dw_hdmi_read_edid(&mut self) {
        todo!();
    }
}

// TODO clock_set_pll3_factors() does a write, this does modify
// might need to add that pattern
fn clock_set_pll3_factors(m: u32, n: u32, _ccu: &mut Ccu) {
    debug_assert_ne!(m & !0xFF, 0);
    debug_assert_ne!(n & !0xFF, 0);

    let ccu = unsafe { &mut *CCU::mut_ptr() };

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

// TODO clock_set_pll3_factors() does a write, this does modify
// might need to add that pattern
fn clock_set_pll3(clk: u32, _ccu: &mut Ccu) {
    let ccu = unsafe { &mut *CCU::mut_ptr() };

    // 6 MHz steps to allow higher frequency for DE2
    let m = 4;

    if clk == 0 {
        ccu.pll_video0.modify(PllVideo0Control::Enable::Clear);
    } else {
        let n = clk / (Clocks::OSC_24M_FREQ / m);
        let factor_n = n - 1;
        let factor_m = m - 1;
        // PLL3 rate = 24000000 * n / m
        ccu.pll_video0.modify(
            PllVideo0Control::Enable::Set
                + PllVideo0Control::Mode::Integer
                + PllVideo0Control::FactorN::Field::new(factor_n).unwrap()
                + PllVideo0Control::PreDivM::Field::new(factor_m).unwrap(),
        );
    }
}

fn delay_ms(ms: usize) {
    delay_us(ms * 1000);
}

// TODO - get a real timer setup
fn delay_us(us: usize) {
    for _ in 0..us {
        for _ in 0..(24 + 10) {
            asm::nop();
        }
    }
}
