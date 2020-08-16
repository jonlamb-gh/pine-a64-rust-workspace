//! Extended display identification data
//!
//! 1.3
//!
//! https://glenwing.github.io/docs/VESA-EEDID-A1.pdf

use nom::{
    number::complete::{be_u16, le_u16, le_u32, le_u64, le_u8},
    IResult,
};

// TODO
// - move this into an HDMI crate
// - Error type https://github.com/Geal/nom/blob/master/examples/custom_error.rs
// - checksum check
// - split up the sub-byte fields using bits mod stuff bits::complete as bits
// - do the unit conversions

// http://www.drhdmi.eu/dictionary/edid.html

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Edid {
    pub header: EdidHeader,
    pub info: BasicDisplayParams,
    pub color_characteristics: ColorCharacteristics,
    pub established_timings: EstablishedTimings,
    pub standard_timings: StandardTimings,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EdidHeader {
    pub manufacturer_name: u16,
    pub product_code: u16,
    pub serial_number: u32,
    pub week: u8,
    pub year: u8,
    pub version: u8,
    pub revision: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BasicDisplayParams {
    pub video_input_definition: u8,
    pub max_size_horizontal: u8,
    pub max_size_vertical: u8,
    pub gamma: u8,
    pub feature_support: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ColorCharacteristics {
    pub red_green: u8,
    pub blue_white: u8,
    pub red_x: u8,
    pub red_y: u8,
    pub green_x: u8,
    pub green_y: u8,
    pub blue_x: u8,
    pub blue_y: u8,
    pub white_point_x: u8,
    pub white_point_y: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EstablishedTimings {
    pub timing_modes: [u8; 3],
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StandardTimings {
    pub display_modes: [StandardTimingInfo; 8],
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StandardTimingInfo {
    pub xresolution: u8,
    pub aspect_vfreq: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DescriptorType {
    SerialNumber = 0xFF,
    Text = 0xFE,
    MonitorRange = 0xFD,
    MonitorName = 0xFC,
}

pub const EDID_SIZE: usize = 128;
pub const EDID_EXT_SIZE: usize = 256;

const HEADER_PREAMBLE: u64 = 0x00_FF_FF_FF_FF_FF_FF_00;

pub fn parse_edid(input: &[u8]) -> IResult<&[u8], Edid> {
    let (input, header) = parse_edid_header(input)?;
    let (input, info) = parse_basic_display_params(input)?;
    let (input, color_characteristics) = parse_color_characteristics(input)?;
    let (input, established_timings) = parse_established_timings(input)?;
    let (input, standard_timings) = parse_standard_timings(input)?;

    Ok((
        input,
        Edid {
            header,
            info,
            color_characteristics,
            established_timings,
            standard_timings,
        },
    ))
}

pub fn parse_edid_header(input: &[u8]) -> IResult<&[u8], EdidHeader> {
    let (input, header) = le_u64(input)?;

    assert_eq!(
        header, HEADER_PREAMBLE,
        "Bad header preabmle: 0x{:X}",
        header
    );

    let (input, manufacturer_name) = be_u16(input)?;
    let (input, product_code) = le_u16(input)?;
    let (input, serial_number) = le_u32(input)?;
    let (input, week) = le_u8(input)?;
    let (input, year) = le_u8(input)?;
    let (input, version) = le_u8(input)?;
    let (input, revision) = le_u8(input)?;

    Ok((
        input,
        EdidHeader {
            manufacturer_name,
            product_code,
            serial_number,
            week,
            year,
            version,
            revision,
        },
    ))
}

fn parse_basic_display_params(input: &[u8]) -> IResult<&[u8], BasicDisplayParams> {
    let (input, video_input_definition) = le_u8(input)?;
    let (input, max_size_horizontal) = le_u8(input)?;
    let (input, max_size_vertical) = le_u8(input)?;
    let (input, gamma) = le_u8(input)?;
    let (input, feature_support) = le_u8(input)?;

    Ok((
        input,
        BasicDisplayParams {
            video_input_definition,
            max_size_horizontal,
            max_size_vertical,
            gamma,
            feature_support,
        },
    ))
}

fn parse_color_characteristics(input: &[u8]) -> IResult<&[u8], ColorCharacteristics> {
    let (input, red_green) = le_u8(input)?;
    let (input, blue_white) = le_u8(input)?;
    let (input, red_x) = le_u8(input)?;
    let (input, red_y) = le_u8(input)?;
    let (input, green_x) = le_u8(input)?;
    let (input, green_y) = le_u8(input)?;
    let (input, blue_x) = le_u8(input)?;
    let (input, blue_y) = le_u8(input)?;
    let (input, white_point_x) = le_u8(input)?;
    let (input, white_point_y) = le_u8(input)?;

    Ok((
        input,
        ColorCharacteristics {
            red_green,
            blue_white,
            red_x,
            red_y,
            green_x,
            green_y,
            blue_x,
            blue_y,
            white_point_x,
            white_point_y,
        },
    ))
}

fn parse_established_timings(input: &[u8]) -> IResult<&[u8], EstablishedTimings> {
    let (input, timing_modes_0) = le_u8(input)?;
    let (input, timing_modes_1) = le_u8(input)?;
    let (input, timing_modes_2) = le_u8(input)?;
    let timing_modes = [timing_modes_0, timing_modes_1, timing_modes_2];

    Ok((input, EstablishedTimings { timing_modes }))
}

fn parse_standard_timings(input: &[u8]) -> IResult<&[u8], StandardTimings> {
    let (input, ti_0) = parse_standard_timing_info(input)?;
    let (input, ti_1) = parse_standard_timing_info(input)?;
    let (input, ti_2) = parse_standard_timing_info(input)?;
    let (input, ti_3) = parse_standard_timing_info(input)?;
    let (input, ti_4) = parse_standard_timing_info(input)?;
    let (input, ti_5) = parse_standard_timing_info(input)?;
    let (input, ti_6) = parse_standard_timing_info(input)?;
    let (input, ti_7) = parse_standard_timing_info(input)?;
    let display_modes = [ti_0, ti_1, ti_2, ti_3, ti_4, ti_5, ti_6, ti_7];

    Ok((input, StandardTimings { display_modes }))
}

fn parse_standard_timing_info(input: &[u8]) -> IResult<&[u8], StandardTimingInfo> {
    let (input, xresolution) = le_u8(input)?;
    let (input, aspect_vfreq) = le_u8(input)?;

    Ok((
        input,
        StandardTimingInfo {
            xresolution,
            aspect_vfreq,
        },
    ))
}
