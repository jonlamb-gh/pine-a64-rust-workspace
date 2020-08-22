//! HDMI display

// TODO - error type, EDID variant, ...

use super::{de2::DisplayEngine2, dw_hdmi::DwHdmi, lcdc::LcdController};
use super::{BitsPerPixel, DisplayTiming};
use crate::ccu::Ccu;
use crate::delay::{delay_ms, delay_us};
use crate::pac::ccu::{
    BusClockGating1, BusSoftReset1, HdmiClockConfig, HdmiSlowClock, Tcon1ClockConfig, CCU,
};
use crate::pac::hdmi::{PhyControl, PhyPll, PhyStatus};
use crate::pac::{de::DE, de_mixer::MIXER1, hdmi::HDMI, tcon1::TCON1};
use core::pin::Pin;
use cortex_a::asm;
use edid::Edid;
use embedded_time::rate::Hertz;

pub mod edid;
pub use super::dw_hdmi::HDMI_EDID_BLOCK_SIZE;

pub struct HdmiDisplay {
    _lcdc: LcdController,
    _de: DisplayEngine2,
    _hdmi: DwHdmi,
    timing: DisplayTiming,
    edid: Edid,
}

impl HdmiDisplay {
    pub fn new(
        tcon: TCON1,
        mixer: MIXER1,
        de: DE,
        mut hdmi: HDMI,
        frame_buffer: &Pin<&'static mut [u32]>,
        ccu: &mut Ccu,
    ) -> Self {
        let fb_addr = frame_buffer.as_ptr() as usize;
        assert!(fb_addr <= core::u32::MAX as usize);
        let fb_addr: u32 = fb_addr as u32;
        let raw_ccu = unsafe { &mut *CCU::mut_ptr() };

        // Set pll3 to 297 MHz
        ccu.set_pll_video0(Hertz::new(297_000_000));

        // Set hdmi parent to pll3
        raw_ccu
            .hdmi_clk_cfg
            .modify(HdmiClockConfig::ClockSel::Pll3Video0x1);

        // Set AHB gating to pass
        ccu.bsr1.rstr().modify(BusSoftReset1::Hdmi1::Clear);
        ccu.bsr1.rstr().modify(BusSoftReset1::Hdmi1::Set);
        ccu.bsr1.rstr().modify(BusSoftReset1::Hdmi0::Clear);
        ccu.bsr1.rstr().modify(BusSoftReset1::Hdmi0::Set);

        ccu.bcg1.enr().modify(BusClockGating1::Hdmi::Set);
        raw_ccu
            .hdmi_slow_clk_cfg
            .modify(HdmiSlowClock::DdcClockGating::Set);

        // Clock on
        raw_ccu
            .hdmi_clk_cfg
            .modify(HdmiClockConfig::SClockGating::Set);

        phy_init(&mut hdmi);

        // TODO - error handle, can't get hpd signal
        wait_for_hpd(&mut hdmi).expect("TODO - Errors");

        let mut hdmi = DwHdmi::new(hdmi);

        // TODO - use the parsed Edid data
        let timing = DisplayTiming::default();
        let bpp: BitsPerPixel = 32;

        let mut edid_block = [0; HDMI_EDID_BLOCK_SIZE];
        hdmi.read_edid_block(&mut edid_block);
        let (_, edid) = edid::parse_edid(&edid_block).expect("TODO errors");

        let mut de = DisplayEngine2::new(mixer, de, ccu);

        de.set_mode(fb_addr, bpp, &timing);

        hdmi.enable(&timing, ccu);

        let lcdc = dw_hdmi_lcdc_init(tcon, &timing, ccu);

        // TODO
        // edid.flags = 10
        // doesn't have h/v sync low bits...

        hdmi.hdmi.phy_ctrl.modify(PhyControl::F2::Full);

        // This is last hdmi access before boot, so scramble addresses
        // again or othwerwise BSP driver won't work. Dummy read is
        // needed or otherwise last write doesn't get written correctly.
        let _ = hdmi.hdmi.version.read();
        hdmi.hdmi.phy_unscramble.write(0);

        delay_ms(5);

        HdmiDisplay {
            _lcdc: lcdc,
            _de: de,
            _hdmi: hdmi,
            timing,
            edid,
        }
    }

    pub fn timing(&self) -> &DisplayTiming {
        &self.timing
    }

    pub fn edid(&self) -> &Edid {
        &self.edid
    }
}

fn wait_for_hpd(hdmi: &mut HDMI) -> Result<(), ()> {
    // TODO - timeout 300 us
    while !hdmi.phy_status.is_set(PhyStatus::PlugIn::Read) {
        delay_us(100);
    }

    Ok(())
}

fn phy_init(hdmi: &mut HDMI) {
    // HDMI PHY settings are taken as-is from Allwinner BSP code.
    // There is no documentation.
    hdmi.phy_ctrl.write(0);
    hdmi.phy_ctrl.modify(PhyControl::B0::Set);
    delay_us(5);
    hdmi.phy_ctrl.modify(PhyControl::B16::Set);
    hdmi.phy_ctrl.modify(PhyControl::B1::Set);
    delay_us(10);
    hdmi.phy_ctrl.modify(PhyControl::B2::Set);
    delay_us(5);
    hdmi.phy_ctrl.modify(PhyControl::B3::Set);
    delay_us(40);
    hdmi.phy_ctrl.modify(PhyControl::B19::Set);
    delay_us(100);
    hdmi.phy_ctrl.modify(PhyControl::B18::Set);
    hdmi.phy_ctrl.modify(PhyControl::F0::Full);

    // Note that Allwinner code doesn't fail in case of timeout
    // PHY_STATUS_TIMEOUT_US = 2000
    while !hdmi.phy_status.is_set(PhyStatus::Ready::Read) {
        asm::nop();
    }

    hdmi.phy_ctrl.modify(PhyControl::F1::Full);
    hdmi.phy_ctrl.modify(PhyControl::B7::Set);

    hdmi.phy_pll.write(0x39dc5040);
    hdmi.phy_clk.write(0x80084343);
    delay_us(10000);
    hdmi.phy_unk3.write(1);
    hdmi.phy_pll.modify(PhyPll::B25::Set);
    delay_us(100000);
    let tmp = (hdmi.phy_status.read() & 0x1_F800) >> 11;
    hdmi.phy_pll.modify(PhyPll::B31::Set + PhyPll::B30::Set);
    hdmi.phy_pll.modify(PhyPll::F0::Field::new(tmp).unwrap());
    hdmi.phy_ctrl.write(0x01FF0F7F);
    hdmi.phy_unk1.write(0x80639000);
    hdmi.phy_unk2.write(0x0F81C405);

    // Enable read access to HDMI controller
    hdmi.phy_read_en.write(0x54524545);

    // Descramble register offsets
    hdmi.phy_unscramble.write(0x42494E47);
}

fn dw_hdmi_lcdc_init(tcon: TCON1, timing: &DisplayTiming, ccu: &mut Ccu) -> LcdController {
    // Assumes mux=1, HDMI
    let div = div_round_up(ccu.pll_video0().0, timing.pixel_clock.typ);
    let mux = 1;

    let raw_ccu = unsafe { &mut *CCU::mut_ptr() };

    if mux == 0 {
        unimplemented!();
    } else {
        // Reset
        ccu.bsr1.rstr().modify(BusSoftReset1::Tcon1::Clear);
        ccu.bsr1.rstr().modify(BusSoftReset1::Tcon1::Set);

        // Clock on
        ccu.bcg1.enr().modify(BusClockGating1::Tcon1::Set);
        raw_ccu.tcon1_clk_cfg.modify(
            Tcon1ClockConfig::DivRatioM::Field::new(div - 1).unwrap()
                + Tcon1ClockConfig::SClockGating::Set,
        );
    }

    let mut lcdc = LcdController::new(tcon);
    lcdc.set_mode(timing);
    lcdc.enable();
    lcdc
}

fn div_round_up(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}
