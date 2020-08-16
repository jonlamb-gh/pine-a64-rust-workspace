use crate::display::HdmiDisplay;
use crate::hal::pac::tcon1::{
    Control, DataClock, GlobalControl, Timing0, Timing1, Timing2, Timing3, Timing4, Timing5,
};

impl<'a> HdmiDisplay<'a> {
    pub(crate) fn lcdc_init(&mut self) {
        self.tcon.gctrl.write(0);
        self.tcon.gint0.write(0);

        // Disable tcon0 dot clock
        self.tcon.tcon0_dclk.modify(DataClock::Enable::Disabled);

        // Set all io lines to tristate
        self.tcon.tcon0_io_trigger.write(0xFFFF_FFFF);
        self.tcon.io_trigger.write(0xFFFF_FFFF);
    }

    pub(crate) fn lcdc_tcon1_mode_set(&mut self) {
        let clk_delay = self.get_clk_delay();

        if self.timing.flags.interlaced() {
            unimplemented!();
        }

        self.tcon
            .ctrl
            .modify(Control::StartDelay::Field::new(clk_delay).unwrap() + Control::Enable::Set);

        let mut yres = self.timing.vactive.typ;
        if self.timing.flags.interlaced() {
            yres /= 2;
        }

        self.tcon.timing_src.modify(
            Timing0::X::Field::new(self.timing.hactive.typ - 1).unwrap()
                + Timing0::Y::Field::new(yres - 1).unwrap(),
        );
        self.tcon.timing_scale.modify(
            Timing1::X::Field::new(self.timing.hactive.typ - 1).unwrap()
                + Timing1::Y::Field::new(yres - 1).unwrap(),
        );
        self.tcon.timing_out.modify(
            Timing2::X::Field::new(self.timing.hactive.typ - 1).unwrap()
                + Timing2::Y::Field::new(yres - 1).unwrap(),
        );

        let bp = self.timing.hsync_len.typ + self.timing.hback_porch.typ;
        let total = self.timing.hactive.typ + self.timing.hfront_porch.typ + bp;
        self.tcon.timing_h.modify(
            Timing3::Hbp::Field::new(bp - 1).unwrap() + Timing3::Ht::Field::new(total - 1).unwrap(),
        );

        let bp = self.timing.vsync_len.typ + self.timing.vback_porch.typ;
        let mut total = self.timing.vactive.typ + self.timing.vfront_porch.typ + bp;
        if !self.timing.flags.interlaced() {
            total = total * 2;
        }

        self.tcon.timing_v.modify(
            Timing4::Vbp::Field::new(bp - 1).unwrap() + Timing4::Vt::Field::new(total).unwrap(),
        );

        self.tcon.timing_sync.modify(
            Timing5::Vspw::Field::new(self.timing.vsync_len.typ - 1).unwrap()
                + Timing5::Hspw::Field::new(self.timing.hsync_len.typ - 1).unwrap(),
        );
    }

    pub(crate) fn lcdc_enable(&mut self, _depth: u32) {
        self.tcon.gctrl.modify(GlobalControl::Enable::Set);
    }

    fn get_clk_delay(&mut self) -> u32 {
        let mut delay =
            self.timing.vfront_porch.typ + self.timing.vsync_len.typ + self.timing.vback_porch.typ;

        if self.timing.flags.interlaced() {
            delay = delay / 2;
        }

        // TODO if tcon == 1
        delay = delay - 2;

        if delay > 30 {
            30
        } else {
            delay
        }
    }
}
