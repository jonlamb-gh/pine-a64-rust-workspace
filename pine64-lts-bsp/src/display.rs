use crate::hal::ccu::{Ccu, Clocks};
use crate::hal::cortex_a::asm;
use crate::hal::pac::ccu::{BusSoftReset1, HdmiClockConfig, PllVideo0Control, CCU};
use crate::hal::pac::hdmi::{Control, Hpd, PllDbg0, HDMI};
use crate::hal::pac::tcon1::TCON1;

// hpd = 1
// hpd_delay = 500
// edid = 1
//
// overscan_x = 0
// overscan_y = 0
//
// x8r8g8b8

// HDMI reg defs in arch/arm/include/asm/arch-sunxi/display.h
// sunxi_hdmi_reg

// arch/arm/include/asm/arch-sunxi/clock_sun6i.h
// obj-$(CONFIG_MACH_SUN50I)   += clock_sun6i.o

// TODO
// - add Ccu abstractions to get rid of unsafe &mut *CCU::mut_ptr()
// - refactor all of the methods/functions
// - add log! debug stuff

// TODO

// doesn't look like drivers/video/sunxi/sunxi_display.c is used ...
//
// sunxi_dw_hdmi.c: Allwinner DW HDMI bridge
//
// but printfs from drivers/video/sunxi/sunxi_dw_hdmi.c are showing up

const HPD_DELAY: usize = 500;

const HDMI_PAD_CTRL0_HDP: u32 = 0xFE80_0000;

const HDMI_PAD_CTRL1: u32 = 0x00D8_C830;
const HDMI_PAD_CTRL1_HALVE: u32 = 1 << 6;

const HDMI_PLL_CTRL: u32 = 0xFA4E_F708;

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

        d.video_hw_init(ccu);

        d
    }

    fn video_hw_init(&mut self, ccu: &mut Ccu) {
        // drivers/video/sunxi/sunxi_display.c
        // video_hw_init()

        let hdmi_present = hdmi_hpd_detect(HPD_DELAY, &mut self.hdmi, ccu);
        // ... if present ...

        // Fall back to EDID in case HPD failed
    }
}

// sunxi_hdmi_hpd_detect()
fn hdmi_hpd_detect(_hpd_delay: usize, hdmi: &mut HDMI, ccu: &mut Ccu) -> bool {
    // Set pll3 to 300MHz
    clock_set_pll3(300_000_000, ccu);

    let ccu = unsafe { &mut *CCU::mut_ptr() };

    // Set hdmi parent to pll3
    ccu.hdmi_clk_cfg
        .modify(HdmiClockConfig::ClockSel::Pll3Video0x1);

    // Set ahb gating to pass
    // TODO - u-boot uses HDMI1 (offset 11)
    // not HDMI0 (offset 10, they call it HDMI2)
    ccu.bsr1.modify(BusSoftReset1::Hdmi1::Clear);
    ccu.bsr1.modify(BusSoftReset1::Hdmi1::Set);

    // Clock on
    ccu.hdmi_clk_cfg.modify(HdmiClockConfig::SClockGating::Set);

    hdmi.ctrl.modify(Control::Enable::Set);
    hdmi.pad_ctrl0.write(HDMI_PAD_CTRL0_HDP);

    // Enable PLLs for eventual DDC
    hdmi.pad_ctrl1.write(HDMI_PAD_CTRL1 | HDMI_PAD_CTRL1_HALVE);
    hdmi.pll_ctrl.write(HDMI_PLL_CTRL | (15 << 4));
    hdmi.pll_dbg0.modify(PllDbg0::Pll::Pll3Video0);

    // TODO - timeout, return false
    while !hdmi.hpd.is_set(Hpd::Detect::Read) {
        asm::nop();
    }

    return true;
}

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
