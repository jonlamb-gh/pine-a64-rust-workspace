//! DesignWare HDMI bridge

use super::DisplayTiming;
use crate::ccu::Ccu;
use crate::delay::delay_ms;
use crate::pac::hdmi::{self, PhyPll, HDMI};

// TODO
// - use Hertz/KiloHertz
// - move the HDMI phy ops to here
//use embedded_time::rate::Hertz;

pub const HDMI_EDID_BLOCK_SIZE: usize = 128;

const BLOCK_0: usize = 0;
const I2C_CLK_HIGH: u8 = 0xD8;
const I2C_CLK_LOW: u8 = 0xFE;
const DIV_STD_MODE: u8 = 0x00;
const DIV_FAST_STD_MODE: u8 = 0x08;
const SLAVE_DDC_ADDR: u8 = 0x50;
const SEGADDR_DDC: u8 = 0x30;
const OP_RD8: u8 = 0x01;

const TX_INVID0_INTERNAL_DE_GENERATOR_DISABLE: u8 = 0x00;
const TX_INVID0_VIDEO_MAPPING_MASK: u8 = 0x1F;
const TX_INVID0_VIDEO_MAPPING_OFFSET: u8 = 0x00;

const TX_INSTUFFING_BDBDATA_STUFFING_ENABLE: u8 = 0x4;
const TX_INSTUFFING_RCRDATA_STUFFING_ENABLE: u8 = 0x02;
const TX_INSTUFFING_GYDATA_STUFFING_ENABLE: u8 = 0x01;

const VP_PR_CD_COLOR_DEPTH_MASK: u8 = 0xF0;
const VP_PR_CD_COLOR_DEPTH_OFFSET: u8 = 0x04;
const VP_PR_CD_DESIRED_PR_FACTOR_MASK: u8 = 0x0F;
const VP_PR_CD_DESIRED_PR_FACTOR_OFFSET: u8 = 0x00;

const VP_STUFF_IDEFAULT_PHASE_MASK: u8 = 0x20;
const VP_STUFF_IDEFAULT_PHASE_OFFSET: u8 = 0x05;
const VP_STUFF_YCC422_STUFFING_MASK: u8 = 0x04;
const VP_STUFF_PP_STUFFING_MASK: u8 = 0x02;
const VP_STUFF_PR_STUFFING_MASK: u8 = 0x01;
const VP_STUFF_PR_STUFFING_STUFFING_MODE: u8 = 0x01;
const VP_STUFF_PP_STUFFING_STUFFING_MODE: u8 = 0x02;
const VP_STUFF_YCC422_STUFFING_STUFFING_MODE: u8 = 0x04;

const VP_CONF_OUTPUT_SELECTOR_BYPASS: u8 = 0x03;
const VP_CONF_PP_EN_ENMASK: u8 = 0x20;
const VP_CONF_BYPASS_EN_MASK: u8 = 0x40;
const VP_CONF_PP_EN_DISABLE: u8 = 0x00;
const VP_CONF_BYPASS_EN_ENABLE: u8 = 0x40;
const VP_CONF_PR_EN_DISABLE: u8 = 0x00;
const VP_CONF_PR_EN_MASK: u8 = 0x10;
const VP_CONF_BYPASS_SELECT_VID_PACKETIZER: u8 = 0x04;
const VP_CONF_YCC422_EN_MASK: u8 = 0x08;
const VP_CONF_YCC422_EN_DISABLE: u8 = 0x00;
const VP_CONF_BYPASS_SELECT_MASK: u8 = 0x04;
const VP_CONF_OUTPUT_SELECTOR_MASK: u8 = 0x03;

const VP_REMAP_YCC422_16BIT: u8 = 0x00;

const FC_INVIDCONF_HDCP_KEEPOUT_INACTIVE: u8 = 0x00;
const FC_INVIDCONF_VSYNC_IN_POLARITY_ACTIVE_HIGH: u8 = 0x40;
const FC_INVIDCONF_VSYNC_IN_POLARITY_ACTIVE_LOW: u8 = 0x00;
const FC_INVIDCONF_HSYNC_IN_POLARITY_ACTIVE_HIGH: u8 = 0x20;
const FC_INVIDCONF_HSYNC_IN_POLARITY_ACTIVE_LOW: u8 = 0x00;
const FC_INVIDCONF_DE_IN_POLARITY_ACTIVE_HIGH: u8 = 0x10;
const FC_INVIDCONF_DE_IN_POLARITY_ACTIVE_LOW: u8 = 0x00;
const FC_INVIDCONF_DVI_MODEZ_HDMI_MODE: u8 = 0x08;
const FC_INVIDCONF_DVI_MODEZ_DVI_MODE: u8 = 0x00;
const FC_INVIDCONF_R_V_BLANK_IN_OSC_ACTIVE_LOW: u8 = 0x00;
const FC_INVIDCONF_IN_I_P_PROGRESSIVE: u8 = 0x0;

const MC_FLOWCTRL_FEED_THROUGH_OFF_CSC_BYPASS: u8 = 0x00;
const MC_CLKDIS_PIXELCLK_DISABLE: u8 = 0x01;
const MC_CLKDIS_TMDSCLK_DISABLE: u8 = 0x02;
const MC_SWRSTZ_TMDSSWRST_REQ: u8 = 0x02;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HdmiReg {
    IhMute,
    IhI2cmStat0,
    TxInvId0,
    TxInStuffing,
    TxGyData0,
    TxGyData1,
    TxRcrData0,
    TxRcrData1,
    TxBcbData0,
    TxBcbData1,
    VpPrCd,
    VpStuff,
    VpRemap,
    VpConf,
    I2cmSlave,
    I2cmAddr,
    I2cmDataI,
    I2cmOp,
    I2cmInt,
    I2cmCtlInt,
    I2cmDiv,
    I2cmSegAddr,
    I2cmSegPtr,
    SsSclHCnt0,
    SsSclLCnt0,
    FcInvIdConf,
    FcInHActv0,
    FcInHActv1,
    FcInHBlank0,
    FcInHBlank1,
    FcInVActv0,
    FcInVActv1,
    FcInVBlank,
    FcHSyncInDelay0,
    FcHSyncInDelay1,
    FcHSyncInWidth0,
    FcHSyncInWidth1,
    FcVSyncInDelay,
    FcVSyncInWidth,
    FcCtrlDur,
    FcExCtrlDur,
    FcExCtrlSpac,
    FcCh0Pream,
    FcCh1Pream,
    FcCh2Pream,
    McFlowCtrl,
    McClkDis,
    McSwRstz,
}

impl HdmiReg {
    fn offset(self) -> usize {
        use HdmiReg::*;
        match self {
            IhMute => 0x01FF,
            IhI2cmStat0 => 0x0105,
            TxInvId0 => 0x0200,
            TxInStuffing => 0x0201,
            TxGyData0 => 0x0202,
            TxGyData1 => 0x0203,
            TxRcrData0 => 0x0204,
            TxRcrData1 => 0x0205,
            TxBcbData0 => 0x0206,
            TxBcbData1 => 0x0207,
            VpPrCd => 0x0801,
            VpStuff => 0x0802,
            VpRemap => 0x0803,
            VpConf => 0x0804,
            I2cmSlave => 0x7E00,
            I2cmAddr => 0x7E01,
            I2cmDataI => 0x7E03,
            I2cmOp => 0x7E04,
            I2cmInt => 0x7E05,
            I2cmCtlInt => 0x7E06,
            I2cmDiv => 0x7E07,
            I2cmSegAddr => 0x7E08,
            I2cmSegPtr => 0x7E0A,
            SsSclHCnt0 => 0x7E0C,
            SsSclLCnt0 => 0x7E0E,
            FcInvIdConf => 0x1000,
            FcInHActv0 => 0x1001,
            FcInHActv1 => 0x1002,
            FcInHBlank0 => 0x1003,
            FcInHBlank1 => 0x1004,
            FcInVActv0 => 0x1005,
            FcInVActv1 => 0x1006,
            FcInVBlank => 0x1007,
            FcHSyncInDelay0 => 0x1008,
            FcHSyncInDelay1 => 0x1009,
            FcHSyncInWidth0 => 0x100A,
            FcHSyncInWidth1 => 0x100B,
            FcVSyncInDelay => 0x100C,
            FcVSyncInWidth => 0x100D,
            FcCtrlDur => 0x1011,
            FcExCtrlDur => 0x1012,
            FcExCtrlSpac => 0x1013,
            FcCh0Pream => 0x1014,
            FcCh1Pream => 0x1015,
            FcCh2Pream => 0x1016,
            McFlowCtrl => 0x4004,
            McClkDis => 0x4001,
            McSwRstz => 0x4002,
        }
    }
}

pub struct DwHdmi {
    pub(crate) hdmi: HDMI,
}

impl DwHdmi {
    pub(crate) fn new(hdmi: HDMI) -> Self {
        use HdmiReg::*;

        let mut h = DwHdmi { hdmi };

        // Disable IH mute interrupts
        h.hdmi_write(IhMute, 0x3);

        // Enable i2c master done irq
        h.hdmi_write(I2cmInt, !0x04);

        // Enable i2c client nack % arbitration error irq
        h.hdmi_write(I2cmCtlInt, !0x44);

        h
    }

    pub(crate) fn enable(&mut self, timing: &DisplayTiming, ccu: &mut Ccu) {
        self.av_composer(timing);

        self.phy_cfg(timing.pixel_clock.typ, ccu);

        self.enable_video_path();

        assert_eq!(timing.hdmi_monitor, false, "TODO - only DVI for now");

        self.video_packetize();
        self.video_csc();
        self.video_sample();

        self.clear_overflow();
    }

    pub(crate) fn read_edid_block(&mut self, edid_block: &mut [u8; HDMI_EDID_BLOCK_SIZE]) {
        self.read_edid(BLOCK_0, edid_block);

        if edid_block[0x7E] != 0 {
            todo!("read EDID ext block")
        }
    }

    // TODO - timeout
    fn read_edid(&mut self, block_index: usize, edid_block: &mut [u8; HDMI_EDID_BLOCK_SIZE]) {
        use HdmiReg::*;

        // Set ddc i2c clk which devided from ddc_clk to 100khz
        self.hdmi_write(SsSclHCnt0, I2C_CLK_HIGH);
        self.hdmi_write(SsSclLCnt0, I2C_CLK_LOW);
        self.hdmi_mod(I2cmDiv, DIV_FAST_STD_MODE, DIV_STD_MODE);

        self.hdmi_write(I2cmSlave, SLAVE_DDC_ADDR);
        self.hdmi_write(I2cmSegAddr, SEGADDR_DDC);
        self.hdmi_write(I2cmSegPtr, 0);
        if block_index != 0 {
            todo!("block >> 1");
        }

        for n in 0..HDMI_EDID_BLOCK_SIZE {
            self.hdmi_write(I2cmAddr, n as u8);

            self.hdmi_write(I2cmOp, OP_RD8);

            self.wait_i2c_done(10);

            edid_block[n] = self.hdmi_read(I2cmDataI);
        }
    }

    fn phy_cfg(&mut self, mpixel_clock: u32, ccu: &mut Ccu) {
        let phy_div = self.pll_set(mpixel_clock / 1000, ccu);
        self.phy_set(mpixel_clock, phy_div);
    }

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

    fn pll_set(&mut self, clk_khz: u32, ccu: &mut Ccu) -> u32 {
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

        // TODO
        //assert_ne!(best_diff, 0);
        assert_ne!(best_div, 0);
        assert_ne!(best_m, 0);
        assert_ne!(best_n, 0);

        ccu.set_pll_video0_factors(best_m, best_n);

        best_div
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

    fn av_composer(&mut self, timing: &DisplayTiming) {
        use HdmiReg::*;

        let hbl = timing.hback_porch.typ + timing.hfront_porch.typ + timing.hsync_len.typ;
        let vbl = timing.vback_porch.typ + timing.vfront_porch.typ + timing.vsync_len.typ;

        // Set up FC_INVIDCONF
        let mut inv_val = FC_INVIDCONF_HDCP_KEEPOUT_INACTIVE;

        if timing.flags.vsync_high() {
            inv_val |= FC_INVIDCONF_VSYNC_IN_POLARITY_ACTIVE_HIGH;
        } else {
            inv_val |= FC_INVIDCONF_VSYNC_IN_POLARITY_ACTIVE_LOW;
        }

        if timing.flags.hsync_high() {
            inv_val |= FC_INVIDCONF_HSYNC_IN_POLARITY_ACTIVE_HIGH;
        } else {
            inv_val |= FC_INVIDCONF_HSYNC_IN_POLARITY_ACTIVE_LOW;
        }

        let mdataenablepolarity = true; // ?
        if mdataenablepolarity {
            inv_val |= FC_INVIDCONF_DE_IN_POLARITY_ACTIVE_HIGH;
        } else {
            inv_val |= FC_INVIDCONF_DE_IN_POLARITY_ACTIVE_LOW;
        }

        if timing.hdmi_monitor {
            inv_val |= FC_INVIDCONF_DVI_MODEZ_HDMI_MODE;
        } else {
            inv_val |= FC_INVIDCONF_DVI_MODEZ_DVI_MODE;
        }

        inv_val |= FC_INVIDCONF_R_V_BLANK_IN_OSC_ACTIVE_LOW;

        inv_val |= FC_INVIDCONF_IN_I_P_PROGRESSIVE;

        self.hdmi_write(FcInvIdConf, inv_val);

        // Set up horizontal active pixel width
        self.hdmi_write(FcInHActv1, (timing.hactive.typ >> 8) as u8);
        self.hdmi_write(FcInHActv0, timing.hactive.typ as u8);

        // Set up vertical active lines
        self.hdmi_write(FcInVActv1, (timing.vactive.typ >> 8) as u8);
        self.hdmi_write(FcInVActv0, timing.vactive.typ as u8);

        // Set up horizontal blanking pixel region width
        self.hdmi_write(FcInHBlank1, (hbl >> 8) as u8);
        self.hdmi_write(FcInHBlank0, hbl as u8);

        // Set up vertical blanking pixel region width
        self.hdmi_write(FcInVBlank, vbl as u8);

        // Set up hsync active edge delay width (in pixel clks)
        self.hdmi_write(FcHSyncInDelay1, (timing.hfront_porch.typ >> 8) as u8);
        self.hdmi_write(FcHSyncInDelay0, timing.hfront_porch.typ as u8);

        // Set up hsync active pulse width (in pixel clks)
        self.hdmi_write(FcVSyncInDelay, timing.vfront_porch.typ as u8);

        // Set up hsync active pulse width (in pixel clks)
        self.hdmi_write(FcHSyncInWidth1, (timing.hsync_len.typ >> 8) as u8);
        self.hdmi_write(FcHSyncInWidth0, timing.hsync_len.typ as u8);

        // Set up vsync active edge delay (in lines)
        self.hdmi_write(FcVSyncInWidth, timing.vsync_len.typ as u8);
    }

    fn enable_video_path(&mut self) {
        use HdmiReg::*;

        // Control period minimum duration
        self.hdmi_write(FcCtrlDur, 12);
        self.hdmi_write(FcExCtrlDur, 32);
        self.hdmi_write(FcExCtrlSpac, 1);

        // Set to fill tmds data channels
        self.hdmi_write(FcCh0Pream, 0x0B);
        self.hdmi_write(FcCh1Pream, 0x16);
        self.hdmi_write(FcCh2Pream, 0x21);

        self.hdmi_write(McFlowCtrl, MC_FLOWCTRL_FEED_THROUGH_OFF_CSC_BYPASS);

        // Enable pixel clock and tmds data path
        let mut clkdis = 0x7F;
        clkdis &= !MC_CLKDIS_PIXELCLK_DISABLE;
        self.hdmi_write(McClkDis, clkdis);

        clkdis &= !MC_CLKDIS_TMDSCLK_DISABLE;
        self.hdmi_write(McClkDis, clkdis);

        // Enable csc path
        // TODO - if is_color_space_conversion

        // Enable color space conversion if needed
        // TODO - if is_color_space_conversion
        self.hdmi_write(McFlowCtrl, MC_FLOWCTRL_FEED_THROUGH_OFF_CSC_BYPASS);
    }

    fn video_packetize(&mut self) {
        use HdmiReg::*;

        let color_depth = 0;
        let remap_size = VP_REMAP_YCC422_16BIT;
        let output_select = VP_CONF_OUTPUT_SELECTOR_BYPASS;

        // Set the packetizer registers
        let val = ((color_depth << VP_PR_CD_COLOR_DEPTH_OFFSET) & VP_PR_CD_COLOR_DEPTH_MASK)
            | ((0 << VP_PR_CD_DESIRED_PR_FACTOR_OFFSET) & VP_PR_CD_DESIRED_PR_FACTOR_MASK);
        self.hdmi_write(VpPrCd, val);

        self.hdmi_mod(
            VpStuff,
            VP_STUFF_PR_STUFFING_MASK,
            VP_STUFF_PR_STUFFING_STUFFING_MODE,
        );

        // Data from pixel repeater block
        let vp_conf = VP_CONF_PR_EN_DISABLE | VP_CONF_BYPASS_SELECT_VID_PACKETIZER;

        self.hdmi_mod(
            VpConf,
            VP_CONF_PR_EN_MASK | VP_CONF_BYPASS_SELECT_MASK,
            vp_conf,
        );

        self.hdmi_mod(
            VpStuff,
            VP_STUFF_IDEFAULT_PHASE_MASK,
            1 << VP_STUFF_IDEFAULT_PHASE_OFFSET,
        );

        self.hdmi_write(VpRemap, remap_size);

        let vp_conf = VP_CONF_BYPASS_EN_ENABLE | VP_CONF_PP_EN_DISABLE | VP_CONF_YCC422_EN_DISABLE;
        self.hdmi_mod(
            VpConf,
            VP_CONF_BYPASS_EN_MASK | VP_CONF_PP_EN_ENMASK | VP_CONF_YCC422_EN_MASK,
            vp_conf,
        );

        self.hdmi_mod(
            VpStuff,
            VP_STUFF_PP_STUFFING_MASK | VP_STUFF_YCC422_STUFFING_MASK,
            VP_STUFF_PP_STUFFING_STUFFING_MODE | VP_STUFF_YCC422_STUFFING_STUFFING_MODE,
        );

        self.hdmi_mod(VpConf, VP_CONF_OUTPUT_SELECTOR_MASK, output_select);
    }

    fn video_csc(&mut self) {
        // TODO - my setup bails out early, need to put this logic back together
    }

    fn video_sample(&mut self) {
        use HdmiReg::*;

        // TODO - handle all of the MEDIA_BUS_FMT_* variants
        // switch on hdmi->hdmi_data.enc_in_bus_format
        let color_format = 0x01;

        let val = TX_INVID0_INTERNAL_DE_GENERATOR_DISABLE
            | ((color_format << TX_INVID0_VIDEO_MAPPING_OFFSET) & TX_INVID0_VIDEO_MAPPING_MASK);
        self.hdmi_write(TxInvId0, val);

        // Enable tx stuffing: when de is inactive, fix the output data to 0
        let val = TX_INSTUFFING_BDBDATA_STUFFING_ENABLE
            | TX_INSTUFFING_RCRDATA_STUFFING_ENABLE
            | TX_INSTUFFING_GYDATA_STUFFING_ENABLE;
        self.hdmi_write(TxInStuffing, val);
        self.hdmi_write(TxGyData0, 0);
        self.hdmi_write(TxGyData1, 0);
        self.hdmi_write(TxRcrData0, 0);
        self.hdmi_write(TxRcrData1, 0);
        self.hdmi_write(TxBcbData0, 0);
        self.hdmi_write(TxBcbData1, 0);
    }

    // Workaround to clear the overflow condition
    fn clear_overflow(&mut self) {
        use HdmiReg::*;

        // TMDS software reset
        self.hdmi_write(McSwRstz, !MC_SWRSTZ_TMDSSWRST_REQ);

        let val = self.hdmi_read(FcInvIdConf);

        for _ in 0..4 {
            self.hdmi_write(FcInvIdConf, val);
        }
    }

    // TODO - timeout
    fn wait_i2c_done(&mut self, _msec: usize) {
        loop {
            let val = self.hdmi_read(HdmiReg::IhI2cmStat0);
            if val & 0x02 != 0 {
                self.hdmi_write(HdmiReg::IhI2cmStat0, val);
                return;
            }
        }
    }

    fn hdmi_write(&mut self, reg: HdmiReg, val: u8) {
        let addr = hdmi::PADDR + reg.offset();
        unsafe { core::ptr::write_volatile(addr as *mut u8, val) };
    }

    fn hdmi_read(&mut self, reg: HdmiReg) -> u8 {
        let addr = hdmi::PADDR + reg.offset();
        let val = unsafe { core::ptr::read_volatile(addr as *const u8) };
        val
    }

    fn hdmi_mod(&mut self, reg: HdmiReg, mask: u8, data: u8) {
        let val = self.hdmi_read(reg) & !mask;
        self.hdmi_write(reg, val | (data & mask));
    }
}
