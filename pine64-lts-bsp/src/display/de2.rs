use crate::display::HdmiDisplay;
use crate::hal::ccu::{Ccu, Clocks};
use crate::hal::console_writeln;
use crate::hal::cortex_a::asm;
use crate::hal::pac::ccu::{BusClockGating1, BusSoftReset1, DeClockConfig, PllDeControl, CCU};
use crate::hal::pac::de::{BusConfig, GateConfig, ResetConfig, SelConfig};
use crate::hal::pac::de_mixer::bld::{InSize, OutputSize};
use crate::hal::pac::de_mixer::global::{GlobalControl, GlobalSize};
use crate::hal::pac::de_mixer::ui::{Attr, OvlSize, Size};
use crate::hal::pac::de_mixer::{NUM_CHANNELS, NUM_CHANNEL_CONFIGS, NUM_UI_CHANNELS};
use crate::hal::pac::sysc::SYSC;

impl<'a, TX: core::fmt::Write> HdmiDisplay<'a, TX> {
    // sunxi_de2_composer_init
    pub(crate) fn de2_composer_init(&mut self, ccu: &mut Ccu) {
        console_writeln!(&mut self.serial, "de2_composer_init");

        // Set SRAM for video use
        let sysc = unsafe { &mut *SYSC::mut_ptr() };
        let val = sysc.sram_ctrl.read();
        let val = val & !(0x01 << 24);
        sysc.sram_ctrl.write(val);

        let val = sysc.sram_ctrl.read();
        console_writeln!(&mut self.serial, "sysc.sram_ctrl = 0x{:X}", val);

        clock_set_pll10(432_000_000, ccu);

        let ccu = unsafe { &mut *CCU::mut_ptr() };

        // Set DE parent to pll10
        ccu.de_clk_cfg.modify(DeClockConfig::ClockSel::PllDe);

        // Set ahb gating to pass
        ccu.bsr1.modify(BusSoftReset1::De::Clear);
        ccu.bsr1.modify(BusSoftReset1::De::Set);
        ccu.bcg1.modify(BusClockGating1::De::Set);

        // Clock on
        ccu.de_clk_cfg.modify(DeClockConfig::SClockGating::Set);
    }

    // sunxi_de2_mode_set
    pub(crate) fn de2_mode_set(&mut self, bpp: u32) {
        console_writeln!(&mut self.serial, "de2_mode_set");

        // TODO -check this addr and the reg it goes into
        let fb_addr = self.frame_buffer.as_ptr() as usize;
        console_writeln!(&mut self.serial, "    fb_addr 0x{:X}", fb_addr);

        assert!(fb_addr <= core::u32::MAX as usize);
        let fb_addr: u32 = fb_addr as u32;

        // Enable clock
        self.de.rst_cfg.modify(ResetConfig::Mux1::Set);
        self.de.gate_cfg.modify(GateConfig::Mux1::Set);
        self.de.bus_cfg.modify(BusConfig::Mux1::Set);

        self.de.sel_cfg.modify(SelConfig::Bit0::Clear);

        self.mixer.global.ctrl.modify(GlobalControl::Enable::Set);
        self.mixer.global.status.write(0);
        self.mixer.global.dbuf.write(1);
        self.mixer.global.size.modify(
            GlobalSize::SizeWidth::Field::new(self.timing.hactive.typ - 1).unwrap()
                + GlobalSize::SizeHeight::Field::new(self.timing.vactive.typ - 1).unwrap(),
        );

        // memset vi block to zero
        for chan_cfg in 0..NUM_CHANNEL_CONFIGS {
            self.mixer.vi.cfg[chan_cfg].attr.write(0);
            self.mixer.vi.cfg[chan_cfg].size.write(0);
            self.mixer.vi.cfg[chan_cfg].coord.write(0);
            for i in 0..3 {
                self.mixer.vi.cfg[chan_cfg].pitch[i].write(0);
                self.mixer.vi.cfg[chan_cfg].top_laddr[i].write(0);
                self.mixer.vi.cfg[chan_cfg].bot_laddr[i].write(0);
            }

            self.mixer.vi.fcolor[chan_cfg].write(0);
        }

        // memset ui blocks to zero
        for ui_block in 0..NUM_UI_CHANNELS {
            for chan_cfg in 0..NUM_CHANNEL_CONFIGS {
                self.mixer.ui[ui_block].cfg[chan_cfg].attr.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].size.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].coord.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].pitch.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].top_laddr.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].bot_laddr.write(0);
                self.mixer.ui[ui_block].cfg[chan_cfg].fcolor.write(0);
            }

            self.mixer.ui[ui_block].top_haddr.write(0);
            self.mixer.ui[ui_block].bot_haddr.write(0);
            self.mixer.ui[ui_block].ovl_size.write(0);
        }

        // memset alpha blending regs to zero
        self.mixer.bld.fcolor_ctl.write(0);
        for ch in 0..NUM_CHANNELS {
            self.mixer.bld.attr[ch].fcolor.write(0);
            self.mixer.bld.attr[ch].in_size.write(0);
            self.mixer.bld.attr[ch].offset.write(0);
        }
        self.mixer.bld.route.write(0);
        self.mixer.bld.premultiply.write(0);
        self.mixer.bld.bkcolor.write(0);
        self.mixer.bld.output_size.write(0);
        for ch in 0..NUM_CHANNELS {
            self.mixer.bld.mode[ch].write(0);
        }
        self.mixer.bld.ck_ctl.write(0);
        self.mixer.bld.ck_cfg.write(0);
        for ch in 0..NUM_CHANNELS {
            self.mixer.bld.ck_max[ch].write(0);
            self.mixer.bld.ck_min[ch].write(0);
        }
        self.mixer.bld.out_ctl.write(0);

        console_writeln!(&mut self.serial, "**** de2_mode_set HERE");

        console_writeln!(
            &mut self.serial,
            "&self.mixer.bld.fcolor_ctl = 0x{:X}",
            &self.mixer.bld.fcolor_ctl as *const _ as usize
        );
        console_writeln!(
            &mut self.serial,
            "&self.mixer.bld.out_ctl = 0x{:X}",
            &self.mixer.bld.out_ctl as *const _ as usize
        );

        self.mixer.bld.fcolor_ctl.write(0x00000101);
        self.mixer.bld.route.write(1);
        self.mixer.bld.premultiply.write(0);
        self.mixer.bld.bkcolor.write(0xff000000);
        self.mixer.bld.mode[0].write(0x03010301);

        self.mixer.bld.output_size.modify(
            OutputSize::SizeWidth::Field::new(self.timing.hactive.typ - 1).unwrap()
                + OutputSize::SizeHeight::Field::new(self.timing.vactive.typ - 1).unwrap(),
        );

        if self.timing.flags.interlaced() {
            unimplemented!();
        }
        self.mixer.bld.out_ctl.write(0);
        self.mixer.bld.ck_ctl.write(0);

        self.mixer.bld.attr[0].fcolor.write(0xff000000);
        self.mixer.bld.attr[0].in_size.modify(
            InSize::SizeWidth::Field::new(self.timing.hactive.typ - 1).unwrap()
                + InSize::SizeHeight::Field::new(self.timing.vactive.typ - 1).unwrap(),
        );

        // Disable all other units
        self.mixer.vsu.ctl.write(0);
        self.mixer.gsu1.ctl.write(0);
        self.mixer.gsu2.ctl.write(0);
        self.mixer.gsu3.ctl.write(0);
        self.mixer.fce.ctl.write(0);
        self.mixer.bws.ctl.write(0);
        self.mixer.lti.ctl.write(0);
        self.mixer.peak.ctl.write(0);
        self.mixer.ase.ctl.write(0);
        self.mixer.fcc.ctl.write(0);

        // Assume not composite, disable CSC
        self.mixer.csc.ctl.write(0);

        // UI[0] at 0x0120_3000
        //panic!("UI[0] at 0x{:X}", &self.mixer.ui[0] as *const _ as usize);

        // TODO - handle other formats
        // 32 => SUNXI_DE2_FORMAT_XRGB_8888
        assert_eq!(bpp, 32);
        self.mixer.ui[0].cfg[0].attr.write(0);
        self.mixer.ui[0].cfg[0]
            .attr
            .modify(Attr::Format::XRgb8888 + Attr::Enable::Set);

        self.mixer.ui[0].cfg[0].size.modify(
            Size::SizeWidth::Field::new(self.timing.hactive.typ - 1).unwrap()
                + Size::SizeHeight::Field::new(self.timing.vactive.typ - 1).unwrap(),
        );
        self.mixer.ui[0].cfg[0].coord.write(0);
        self.mixer.ui[0].cfg[0]
            .pitch
            .write((bpp / 8) * self.timing.hactive.typ);
        self.mixer.ui[0].cfg[0].top_laddr.write(fb_addr);
        self.mixer.ui[0].ovl_size.modify(
            OvlSize::SizeWidth::Field::new(self.timing.hactive.typ - 1).unwrap()
                + OvlSize::SizeHeight::Field::new(self.timing.vactive.typ - 1).unwrap(),
        );

        // Apply settings
        self.mixer.global.dbuf.write(1);
    }
}

fn clock_set_pll10(clk: u32, _ccu: &mut Ccu) {
    let ccu = unsafe { &mut *CCU::mut_ptr() };

    // 12 MHz steps
    let m = 2;
    if clk == 0 {
        ccu.pll_de.modify(PllDeControl::Enable::Clear);
    } else {
        let n = clk / (Clocks::OSC_24M_FREQ / m);
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
