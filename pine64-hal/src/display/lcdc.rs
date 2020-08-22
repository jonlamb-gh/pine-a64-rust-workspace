//! LCD controller

// TODO - generic over TCON0/TCON1

use super::DisplayTiming;
use crate::pac::tcon1::{
    Control, DataClock, GlobalControl, Timing0, Timing1, Timing2, Timing3, Timing4, Timing5, TCON1,
};

pub struct LcdController {
    tcon: TCON1,
}

impl LcdController {
    pub(crate) fn new(mut tcon: TCON1) -> Self {
        tcon.gctrl.write(0);
        tcon.gint0.write(0);

        // Disable tcon0 dot clock
        tcon.tcon0_dclk.modify(DataClock::Enable::Disabled);

        // Set all io lines to tristate
        tcon.tcon0_io_trigger.write(0xFFFF_FFFF);
        tcon.io_trigger.write(0xFFFF_FFFF);

        LcdController { tcon }
    }

    pub(crate) fn set_mode(&mut self, timing: &DisplayTiming) {
        let clk_delay = self.clock_delay(timing);

        if timing.flags.interlaced() {
            unimplemented!();
        }

        self.tcon
            .ctrl
            .modify(Control::StartDelay::Field::new(clk_delay).unwrap() + Control::Enable::Set);

        let mut yres = timing.vactive.typ;
        if timing.flags.interlaced() {
            yres /= 2;
        }

        self.tcon.timing_src.modify(
            Timing0::X::Field::new(timing.hactive.typ - 1).unwrap()
                + Timing0::Y::Field::new(yres - 1).unwrap(),
        );
        self.tcon.timing_scale.modify(
            Timing1::X::Field::new(timing.hactive.typ - 1).unwrap()
                + Timing1::Y::Field::new(yres - 1).unwrap(),
        );
        self.tcon.timing_out.modify(
            Timing2::X::Field::new(timing.hactive.typ - 1).unwrap()
                + Timing2::Y::Field::new(yres - 1).unwrap(),
        );

        let bp = timing.hsync_len.typ + timing.hback_porch.typ;
        let total = timing.hactive.typ + timing.hfront_porch.typ + bp;
        self.tcon.timing_h.modify(
            Timing3::Hbp::Field::new(bp - 1).unwrap() + Timing3::Ht::Field::new(total - 1).unwrap(),
        );

        let bp = timing.vsync_len.typ + timing.vback_porch.typ;
        let mut total = timing.vactive.typ + timing.vfront_porch.typ + bp;
        if !timing.flags.interlaced() {
            total = total * 2;
        }

        self.tcon.timing_v.modify(
            Timing4::Vbp::Field::new(bp - 1).unwrap() + Timing4::Vt::Field::new(total).unwrap(),
        );

        self.tcon.timing_sync.modify(
            Timing5::Vspw::Field::new(timing.vsync_len.typ - 1).unwrap()
                + Timing5::Hspw::Field::new(timing.hsync_len.typ - 1).unwrap(),
        );
    }

    pub(crate) fn enable(&mut self) {
        self.tcon.gctrl.modify(GlobalControl::Enable::Set);
    }

    fn clock_delay(&mut self, timing: &DisplayTiming) -> u32 {
        let mut delay = timing.vfront_porch.typ + timing.vsync_len.typ + timing.vback_porch.typ;

        if timing.flags.interlaced() {
            delay = delay / 2;
        }

        // TODO if tcon == 1 ...
        delay = delay - 2;

        if delay > 30 {
            30
        } else {
            delay
        }
    }
}
