use crate::hal::ccu::{Ccu, Clocks};
use crate::hal::cortex_a::asm;
use crate::hal::pac::ccu::{
    BusClockGating1, BusSoftReset1, HdmiClockConfig, HdmiSlowClock, PllVideo0Control, CCU,
};
use crate::hal::pac::de::DE;
use crate::hal::pac::de_mixer::MIXER1;
use crate::hal::pac::hdmi::{PhyControl, PhyPll, PhyStatus, HDMI};
use crate::hal::pac::tcon1::TCON1;
use crate::hal::{console_write, console_writeln};
use bitfield::bitfield;

mod de2;
mod dw_hdmi;
mod lcdc;

pub use dw_hdmi::HDMI_EDID_BLOCK_SIZE;

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
// - EDID parser; https://en.wikipedia.org/wiki/Extended_Display_Identification_Data
//   * wire-like type

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

// dw_hdmi_enable dvi, mode info : clock 148500000 hdis 1920 vdis 1080
// edid->pixelclock.typ = 148500000
// edid->hactive.typ = 1920
// edid->vactive.typ = 1080

//------------------ EDID-----
//pixelclock: min 148500000, typ 148500000, max 148500000
//hactive: min 1920, typ 1920, max 1920
//hfront_porch: min 88, typ 88, max 88
//hback_porch: min 148, typ 148, max 148
//hsync_len: min 44, typ 44, max 44
//vactive: min 1080, typ 1080, max 1080
//vfront_porch: min 4, typ 4, max 4
//vback_porch: min 36, typ 36, max 36
//vsync_len: min 5, typ 5, max 5
//flags: 0xA (10)
//hdmi_monitor 0
//
//hdmi_data.enc_in_bus_format: 0 (rbg 4:4:4 ?)
//hdmi_data.enc_out_bus_format: 0
//is_color_space_conversion 0

bitfield! {
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct DisplayFlags(u16);
    u16;
    pub hsync_low, set_hsync_low : 0;
    pub hsync_high, set_hsync_high : 1;
    pub vsync_low, set_vsync_low : 2;
    pub vsync_high, set_vsync_high : 3;

    // Data enable
    pub de_low, set_de_low : 4;
    pub de_high, set_de_high : 5;

    // Drive data on positive edge
    pub pixdata_posedge, set_pixdata_posedge : 6;
    // Drive data on negative edge
    pub pixdata_negedge, set_pixdata_negedge : 7;

    pub interlaced, set_interlaced : 8;
    pub double_scane, set_double_scan : 9;
    pub double_clock, set_double_clock : 10;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TimingEntry {
    pub min: u32,
    pub typ: u32,
    pub max: u32,
}

impl TimingEntry {
    pub fn new(min: u32, typ: u32, max: u32) -> Self {
        TimingEntry { min, typ, max }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DisplayTiming {
    pub pixel_clock: TimingEntry,

    pub hactive: TimingEntry,
    pub hfront_porch: TimingEntry,
    pub hback_porch: TimingEntry,
    pub hsync_len: TimingEntry,

    pub vactive: TimingEntry,
    pub vfront_porch: TimingEntry,
    pub vback_porch: TimingEntry,
    pub vsync_len: TimingEntry,

    pub flags: DisplayFlags,
    pub hdmi_monitor: bool,
}

impl Default for DisplayTiming {
    fn default() -> Self {
        DisplayTiming {
            pixel_clock: TimingEntry::new(148500000, 148500000, 148500000),

            hactive: TimingEntry::new(1920, 1920, 1920),
            hfront_porch: TimingEntry::new(88, 88, 88),
            hback_porch: TimingEntry::new(148, 148, 148),
            hsync_len: TimingEntry::new(44, 44, 44),

            vactive: TimingEntry::new(1080, 1080, 1080),
            vfront_porch: TimingEntry::new(4, 4, 4),
            vback_porch: TimingEntry::new(36, 36, 36),
            vsync_len: TimingEntry::new(5, 5, 5),

            flags: DisplayFlags(0x0A),
            hdmi_monitor: false,
        }
    }
}

pub struct HdmiDisplay<'a, TX> {
    serial: TX,
    tcon: TCON1,
    mixer: MIXER1,
    de: DE,
    hdmi: HDMI,
    timing: DisplayTiming,
    edid_block: [u8; HDMI_EDID_BLOCK_SIZE],
    frame_buffer: &'a mut [u32],
}

impl<'a, TX: core::fmt::Write> HdmiDisplay<'a, TX> {
    pub fn new(
        serial: TX,
        tcon: TCON1,
        mixer: MIXER1,
        de: DE,
        hdmi: HDMI,
        edid_block: [u8; HDMI_EDID_BLOCK_SIZE],
        frame_buffer: &'a mut [u32],
        ccu: &mut Ccu,
    ) -> Self {
        // TODO - checks/etc
        //
        // print out the control regs
        //
        // - PPLs, 1x vs 2x
        // - CCM/CCU
        //
        // ok stuff:
        // - lcdc tcon0/1 regs match up
        // - de base regs
        // - mixer1.global
        // - mixer1.bld

        // B1, B3: HSYNC_HIGH, VSYNC_HIGH
        //let timing = DisplayTiming {
        //    hdmi_monitor: true,
        //};

        let mut d = HdmiDisplay {
            serial,
            tcon,
            mixer,
            de,
            hdmi,
            // TODO - use the data read from hdmi_read_edid()
            timing: DisplayTiming::default(),
            edid_block,
            frame_buffer,
        };

        console_writeln!(&mut d.serial, "Timing: {:#?}", d.timing);

        for pixel in d.frame_buffer.iter_mut() {
            *pixel = 0;
        }

        // sunxi_dw_hdmi_probe()
        d.probe(ccu);

        d.dw_hdmi_read_edid();
        console_writeln!(&mut d.serial, "edid block size {}", d.edid_block.len());
        for (idx, b) in d.edid_block.iter().enumerate() {
            if idx % 8 == 0 {
                console_write!(&mut d.serial, "[{}]: ", idx);
            }

            console_write!(&mut d.serial, "{:02X} ", b);

            if (idx + 1) % 8 == 0 {
                console_writeln!(&mut d.serial);
            }
        }
        console_writeln!(&mut d.serial);

        console_writeln!(&mut d.serial, "---------");
        console_writeln!(&mut d.serial, "---------");

        let (_, edid) = crate::edid::parse_edid(&d.edid_block).unwrap();
        console_writeln!(&mut d.serial, "{:#?}", edid);

        // sunxi_de2_composer_init();
        d.de2_composer_init(ccu);

        // sunxi_de2_mode_set(mux, &timing, 1 << l2bpp, fbbase, is_composite);
        d.de2_mode_set(32);

        console_writeln!(&mut d.serial, "OUT of sunxi_de2_mode_set");

        console_writeln!(&mut d.serial, "ABOUT to call sunxi_dw_hdmi_enable");

        // PanelBpp
        d.enable(32, ccu);

        // TODO
        // video_set_flush_dcache(dev, 1)

        delay_ms(1000);

        // bld
        //let base = 0x0120_0000; // MIXER1
        let base = 0x0120_1000;
        for offset in (0..0x224).step_by(4) {
            let addr = base + offset;
            let val = unsafe { core::ptr::read_volatile(addr as *const u32) };
            console_writeln!(&mut d.serial, "0x{:X} == 0x{:08X}", addr, val);
        }

        console_writeln!(&mut d.serial, "Drawing pixels");

        for _ in 0..1000 {
            for pixel in d.frame_buffer.iter_mut() {
                //*pixel = 0xFF_00_00_FF; // Blue
                *pixel = 0xFF_FF_00_00; // Green
                delay_ms(1);
            }
        }

        // DIFFS
        //
        // mixer1.

        //for _ in 0..1000 {
        //    let base = d.frame_buffer.as_ptr() as usize;
        //    for offset in (0..(d.frame_buffer.len() * 4)).step_by(4) {
        //        let addr = base + offset;
        //        let val = unsafe { core::ptr::write_volatile(addr as *mut u32,
        // 0xFF_00_00_FF) };    }
        //}

        console_writeln!(&mut d.serial, "Display created");

        d
    }

    // sunxi_dw_hdmi_probe
    // drivers/video/sunxi/sunxi_dw_hdmi.c
    fn probe(&mut self, hal_provided_ccu: &mut Ccu) {
        console_writeln!(&mut self.serial, "sunxi_dw_hdmi_probe");

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

        self.dw_hdmi_init();
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
        console_writeln!(&mut self.serial, "sunxi_dw_hdmi_phy_init");
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
        self.hdmi.phy_ctrl.modify(PhyControl::F0::Full);

        // Note that Allwinner code doesn't fail in case of timeout
        // PHY_STATUS_TIMEOUT_US = 2000
        while !self.hdmi.phy_status.is_set(PhyStatus::Ready::Read) {
            asm::nop();
        }

        self.hdmi.phy_ctrl.modify(PhyControl::F1::Full);
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
        console_writeln!(&mut self.serial, "sunxi_dw_hdmi_phy_cfg");
        let phy_div = self.pll_set(mpixel_clock / 1000, hal_provided_ccu);
        self.phy_set(mpixel_clock, phy_div);
    }

    // sunxi_dw_hdmi_pll_set()
    fn pll_set(&mut self, clk_khz: u32, hal_provided_ccu: &mut Ccu) -> u32 {
        console_writeln!(&mut self.serial, "sunxi_dw_hdmi_pll_set");
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
        console_writeln!(
            &mut self.serial,
            "sunxi_dw_hdmi_phy_set clock {}, phy_div {}",
            clock,
            phy_div
        );

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

    // struct display_timing *edid
    // sunxi_dw_hdmi_enable()
    fn enable(&mut self, panel_bpp: u32, ccu: &mut Ccu) {
        console_writeln!(&mut self.serial, "sunxi_dw_hdmi_enable");

        self.dw_hdmi_enable(ccu);

        // mux = 1, hdmi
        // sunxi_dw_hdmi_lcdc_init(mux, edid, panel_bpp);
        self.dw_hdmi_lcdc_init(panel_bpp, ccu);

        // edid.flags = 10
        // doesn't have h/v sync low bits...

        self.hdmi.phy_ctrl.modify(PhyControl::F2::Full);

        // This is last hdmi access before boot, so scramble addresses
        // again or othwerwise BSP driver won't work. Dummy read is
        // needed or otherwise last write doesn't get written correctly.
        let _ = self.hdmi.version.read();
        self.hdmi.phy_unscramble.write(0);
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

        while !ccu.pll_video0.is_set(PllVideo0Control::Lock::Read) {
            asm::nop();
        }
    }
}

pub(crate) fn clock_get_pll3(_ccu: &mut Ccu) -> u32 {
    let ccu = unsafe { &mut *CCU::mut_ptr() };

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

    (24000 * n / m) * 1000
}

pub(crate) fn delay_ms(ms: usize) {
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
