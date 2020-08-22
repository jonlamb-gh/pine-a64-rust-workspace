//! Display pipeline

// TODO
// - Display trait to be impl'd by HdmiDisplay and LcdDisplay
// - proper Delay trait and impl from a timer

use bitfield::bitfield;

mod de2;
mod dw_hdmi;
pub mod hdmi;
mod lcdc;

pub type BitsPerPixel = u32;

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

// TODO - this is a u-boot idiom, only the typ is used
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

// TODO - rm this after parsing EDID info
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
