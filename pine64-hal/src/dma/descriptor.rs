//! DMA LLI descriptor
//!
//! Note: Descriptor address must be half-word aligned

use bitfield::bitfield;
use static_assertions::{assert_eq_align, assert_eq_size, const_assert_eq};

pub const DESCRIPTOR_SIZE: usize = 6 * 4;

assert_eq_size!(Descriptor, [u32; 6]);
const_assert_eq!(DESCRIPTOR_SIZE, 24);
assert_eq_align!(Descriptor, u32);

#[derive(Debug)]
#[repr(C, align(4))]
pub struct Descriptor {
    pub config: Config,
    pub src_addr: u32,
    pub dst_addr: u32,
    /// Length in bytes
    pub length: u32,
    pub param: Param,
    pub next_addr: u32,
}

impl Descriptor {
    pub const LAST_ADDR: u32 = 0xFFFF_F800;

    pub const fn new() -> Self {
        Descriptor {
            config: Config(0),
            src_addr: 0,
            dst_addr: 0,
            length: 0,
            param: Param(0),
            next_addr: Self::LAST_ADDR,
        }
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const Self {
        self as *const _
    }
}

impl Default for Descriptor {
    fn default() -> Self {
        let mut config = Config(0);
        config.set_src_drq_port(DrqPort::SdRam);
        config.set_dst_drq_port(DrqPort::SdRam);
        config.set_src_address_mode(AddressMode::Linear);
        config.set_dst_address_mode(AddressMode::Linear);
        config.set_src_burst_length(BurstLength::Bytes4);
        config.set_dst_burst_length(BurstLength::Bytes4);
        config.set_src_data_width(DataWidth::Bits8);
        config.set_dst_data_width(DataWidth::Bits8);

        let mut param = Param(0);
        param.set_wait(Param::NORMAL_WAIT);

        Descriptor {
            config,
            src_addr: 0,
            dst_addr: 0,
            length: 0,
            param,
            next_addr: Self::LAST_ADDR,
        }
    }
}

bitfield! {
    #[repr(transparent)]
    pub struct Param(u32);
    impl Debug;
    u32;
    /// Wait clock cycle
    pub wait, set_wait : 7, 0;
}

impl Param {
    pub const NORMAL_WAIT: u32 = 8;
}

bitfield! {
    #[repr(transparent)]
    pub struct Config(u32);
    impl Debug;
    u32;
    /// Source DRQ type
    pub src_drq, set_src_drq : 4, 0;
    /// Source address mode
    pub src_mode, set_src_mode : 5;
    /// Source burst length
    pub src_burst, set_src_burst: 7, 6;
    /// Source data width
    pub src_width, set_src_width: 10, 9;
    /// Destination DRQ type
    pub dst_drq, set_dst_drq : 20, 16;
    /// Destination address mode
    pub dst_mode, set_dst_mode : 21;
    /// Destination burst size
    pub dst_burst, set_dst_burst: 23, 22;
    /// Destination data width
    pub dst_width, set_dst_width: 26, 25;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AddressMode {
    Linear,
    Io,
}

/// Burst length in bytes
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BurstLength {
    Bytes1,
    Bytes4,
    Bytes8,
    Bytes16,
}

/// Data width in bits
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DataWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DrqPort {
    SdRam,
}

impl Config {
    const ADDR_MODE_LINEAR: bool = false;
    const ADDR_MODE_IO: bool = true;

    pub fn set_src_drq_port(&mut self, drq: DrqPort) {
        match drq {
            DrqPort::SdRam => self.set_src_drq(1),
        }
    }

    pub fn set_dst_drq_port(&mut self, drq: DrqPort) {
        match drq {
            DrqPort::SdRam => self.set_dst_drq(1),
        }
    }

    pub fn set_src_address_mode(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Linear => self.set_src_mode(Self::ADDR_MODE_LINEAR),
            AddressMode::Io => self.set_src_mode(Self::ADDR_MODE_IO),
        }
    }

    pub fn set_dst_address_mode(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Linear => self.set_dst_mode(Self::ADDR_MODE_LINEAR),
            AddressMode::Io => self.set_dst_mode(Self::ADDR_MODE_IO),
        }
    }

    pub fn set_src_burst_length(&mut self, burst: BurstLength) {
        match burst {
            BurstLength::Bytes1 => self.set_src_burst(0b00),
            BurstLength::Bytes4 => self.set_src_burst(0b01),
            BurstLength::Bytes8 => self.set_src_burst(0b10),
            BurstLength::Bytes16 => self.set_src_burst(0b11),
        }
    }

    pub fn set_dst_burst_length(&mut self, burst: BurstLength) {
        match burst {
            BurstLength::Bytes1 => self.set_dst_burst(0b00),
            BurstLength::Bytes4 => self.set_dst_burst(0b01),
            BurstLength::Bytes8 => self.set_dst_burst(0b10),
            BurstLength::Bytes16 => self.set_dst_burst(0b11),
        }
    }

    pub fn set_src_data_width(&mut self, width: DataWidth) {
        match width {
            DataWidth::Bits8 => self.set_src_width(0b00),
            DataWidth::Bits16 => self.set_src_width(0b01),
            DataWidth::Bits32 => self.set_src_width(0b10),
            DataWidth::Bits64 => self.set_src_width(0b11),
        }
    }

    pub fn set_dst_data_width(&mut self, width: DataWidth) {
        match width {
            DataWidth::Bits8 => self.set_dst_width(0b00),
            DataWidth::Bits16 => self.set_dst_width(0b01),
            DataWidth::Bits32 => self.set_dst_width(0b10),
            DataWidth::Bits64 => self.set_dst_width(0b11),
        }
    }
}
