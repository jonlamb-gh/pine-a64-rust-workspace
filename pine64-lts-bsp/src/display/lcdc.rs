use crate::display::HdmiDisplay;
use crate::hal::console_writeln;
use crate::hal::pac::tcon1::{
    Control, DataClock, GlobalControl, Timing0, Timing1, Timing2, Timing3, Timing4, Timing5,
};

impl<'a, TX: core::fmt::Write> HdmiDisplay<'a, TX> {
    pub(crate) fn lcdc_init(&mut self) {
        console_writeln!(&mut self.serial, "lcdc_init");

        self.tcon.gctrl.write(0);
        self.tcon.gint0.write(0);

        // TODO - u-boot does this?
        // Disable tcon0 dot clock
        self.tcon.tcon0_dclk.modify(DataClock::Enable::Disabled);

        // Set all io lines to tristate
        self.tcon.tcon0_io_trigger.write(0xFFFF_FFFF);
        self.tcon.io_trigger.write(0xFFFF_FFFF);
    }

    // lcdc_tcon1_mode_set
    pub(crate) fn lcdc_tcon1_mode_set(&mut self) {
        console_writeln!(&mut self.serial, "lcdc_tcon1_mode_set");

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

        console_writeln!(&mut self.serial, "timing_v bp {}, total {}", bp, total);

        self.tcon.timing_v.modify(
            Timing4::Vbp::Field::new(bp - 1).unwrap() + Timing4::Vt::Field::new(total).unwrap(),
        );

        self.tcon.timing_sync.modify(
            Timing5::Vspw::Field::new(self.timing.vsync_len.typ - 1).unwrap()
                + Timing5::Hspw::Field::new(self.timing.hsync_len.typ - 1).unwrap(),
        );
    }

    // lcdc_enable
    pub(crate) fn lcdc_enable(&mut self, _depth: u32) {
        console_writeln!(&mut self.serial, "lcdc_enable");

        self.tcon.gctrl.modify(GlobalControl::Enable::Set);

        crate::display::delay_ms(500);

        for _ in 0..1 {
            let val = self.tcon.gctrl.read();
            console_writeln!(&mut self.serial, "tcon1.gctrl = 0x{:X}", val);
            let val = self.tcon.gint0.read();
            console_writeln!(&mut self.serial, "tcon1.gint0 = 0x{:X}", val);
            let val = self.tcon.gint1.read();
            console_writeln!(&mut self.serial, "tcon1.gint1 = 0x{:X}", val);
            let val = self.tcon.ctrl.read();
            console_writeln!(&mut self.serial, "tcon1.ctrl = 0x{:X}", val);
            let val = self.tcon.timing_src.read();
            console_writeln!(&mut self.serial, "tcon1.timing_src = 0x{:X}", val);
            let val = self.tcon.timing_v.read();
            console_writeln!(&mut self.serial, "tcon1.timing_v = 0x{:X}", val);
            let val = self.tcon.timing_h.read();
            console_writeln!(&mut self.serial, "tcon1.timing_h = 0x{:X}", val);
        }

        //let tcon1_base = 0x01C0_D000;
        //for offset in (0..0x224).step_by(4) {
        //    let addr = tcon1_base + offset;
        //    let val = unsafe { core::ptr::read_volatile(addr as *const u32) };
        //    console_writeln!(&mut self.serial, "0x{:X} == 0x{:08X}", addr,
        // val);
        //}
    }

    // lcdc_get_clk_delay
    fn get_clk_delay(&mut self) -> u32 {
        let mut delay =
            self.timing.vfront_porch.typ + self.timing.vsync_len.typ + self.timing.vback_porch.typ;

        if self.timing.flags.interlaced() {
            delay = delay / 2;
        }

        // if tcon == 1
        delay = delay - 2;

        if delay > 30 {
            30
        } else {
            delay
        }
    }
}
