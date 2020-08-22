//! Display engine 2.0

// TODO - SYSC abstractions

use super::{BitsPerPixel, DisplayTiming};
use crate::ccu::Ccu;
use crate::pac::ccu::{BusClockGating1, BusSoftReset1, DeClockConfig};
use crate::pac::de::{BusConfig, GateConfig, ResetConfig, SelConfig};
use crate::pac::de_mixer::{
    bld::{InSize, OutputSize},
    global::{GlobalControl, GlobalSize},
    ui::{Attr, OvlSize, Size},
    NUM_CHANNELS, NUM_CHANNEL_CONFIGS, NUM_UI_CHANNELS,
};
use crate::pac::{ccu::CCU, de::DE, de_mixer::MIXER1, sysc::SYSC};
use embedded_time::rate::Hertz;

pub struct DisplayEngine2 {
    mixer: MIXER1,
    de: DE,
}

impl DisplayEngine2 {
    pub(crate) fn new(mixer: MIXER1, de: DE, ccu: &mut Ccu) -> Self {
        // Set SRAM for video use
        let sysc = unsafe { &mut *SYSC::mut_ptr() };
        let val = sysc.sram_ctrl.read();
        let val = val & !(0x01 << 24);
        sysc.sram_ctrl.write(val);

        ccu.set_pll_de(Hertz::new(432_000_000));

        let raw_ccu = unsafe { &mut *CCU::mut_ptr() };

        // Set DE parent to pll10
        raw_ccu.de_clk_cfg.modify(DeClockConfig::ClockSel::PllDe);

        // Set ahb gating to pass
        ccu.bsr1.rstr().modify(BusSoftReset1::De::Clear);
        ccu.bsr1.rstr().modify(BusSoftReset1::De::Set);
        ccu.bcg1.enr().modify(BusClockGating1::De::Set);

        // Clock on
        raw_ccu.de_clk_cfg.modify(DeClockConfig::SClockGating::Set);

        DisplayEngine2 { mixer, de }
    }

    pub(crate) fn set_mode(&mut self, fb_addr: u32, bpp: BitsPerPixel, timing: &DisplayTiming) {
        // Enable clock
        self.de.rst_cfg.modify(ResetConfig::Mux1::Set);
        self.de.gate_cfg.modify(GateConfig::Mux1::Set);
        self.de.bus_cfg.modify(BusConfig::Mux1::Set);

        self.de.sel_cfg.modify(SelConfig::Bit0::Clear);

        self.mixer.global.ctrl.modify(GlobalControl::Enable::Set);
        self.mixer.global.status.write(0);
        self.mixer.global.dbuf.write(1);
        self.mixer.global.size.modify(
            GlobalSize::SizeWidth::Field::new(timing.hactive.typ - 1).unwrap()
                + GlobalSize::SizeHeight::Field::new(timing.vactive.typ - 1).unwrap(),
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

        self.mixer.bld.fcolor_ctl.write(0x00000101);
        self.mixer.bld.route.write(1);
        self.mixer.bld.premultiply.write(0);
        self.mixer.bld.bkcolor.write(0xff000000);
        self.mixer.bld.mode[0].write(0x03010301);

        self.mixer.bld.output_size.modify(
            OutputSize::SizeWidth::Field::new(timing.hactive.typ - 1).unwrap()
                + OutputSize::SizeHeight::Field::new(timing.vactive.typ - 1).unwrap(),
        );

        if timing.flags.interlaced() {
            unimplemented!();
        }
        self.mixer.bld.out_ctl.write(0);
        self.mixer.bld.ck_ctl.write(0);

        self.mixer.bld.attr[0].fcolor.write(0xff000000);
        self.mixer.bld.attr[0].in_size.modify(
            InSize::SizeWidth::Field::new(timing.hactive.typ - 1).unwrap()
                + InSize::SizeHeight::Field::new(timing.vactive.typ - 1).unwrap(),
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

        // TODO - handle other formats
        // 32 => SUNXI_DE2_FORMAT_XRGB_8888
        assert_eq!(bpp, 32);
        self.mixer.ui[0].cfg[0].attr.write(0);
        self.mixer.ui[0].cfg[0]
            .attr
            .modify(Attr::Format::XRgb8888 + Attr::Enable::Set);

        self.mixer.ui[0].cfg[0].size.modify(
            Size::SizeWidth::Field::new(timing.hactive.typ - 1).unwrap()
                + Size::SizeHeight::Field::new(timing.vactive.typ - 1).unwrap(),
        );
        self.mixer.ui[0].cfg[0].coord.write(0);
        self.mixer.ui[0].cfg[0]
            .pitch
            .write((bpp / 8) * timing.hactive.typ);
        self.mixer.ui[0].cfg[0].top_laddr.write(fb_addr);
        self.mixer.ui[0].ovl_size.modify(
            OvlSize::SizeWidth::Field::new(timing.hactive.typ - 1).unwrap()
                + OvlSize::SizeHeight::Field::new(timing.vactive.typ - 1).unwrap(),
        );

        // Apply settings
        self.mixer.global.dbuf.write(1);
    }
}
