use crate::display::HdmiDisplay;
use crate::hal::pac::hdmi::{Rx1fc, Rx7e04, Rx7e08, Rx7e0c};

pub const HDMI_EDID_BLOCK_SIZE: usize = 128;

const I2C_CLK_HIGH: u8 = 0xD8;
const I2C_CLK_LOW: u8 = 0xFE;
const BLOCK_0: usize = 0;

impl<'a> HdmiDisplay<'a> {
    pub(crate) fn dw_hdmi_init(&mut self) {
        // Disable IH mute interrupts
        self.hdmi.r1fc.modify(Rx1fc::IhMute::Disabled);

        // Enable i2c master done irq
        let val: u8 = !0x04;
        self.hdmi
            .r7e04
            .modify(Rx7e04::I2cmInt::Field::new(val as _).unwrap());

        // Enable i2c client nack % arbitration error irq
        let val: u8 = !0x44;
        self.hdmi
            .r7e04
            .modify(Rx7e04::I2cmCtlInt::Field::new(val as _).unwrap());
    }

    // dw_hdmi_enable()
    pub(crate) fn dw_hdmi_enable(&mut self) {
        todo!();
    }

    // sunxi_dw_hdmi_read_edid()
    // just calls into dw_hdmi_read_edid()
    pub(crate) fn dw_hdmi_read_edid(&mut self) {
        self.hdmi_read_edid(BLOCK_0);

        // slice [u8]

        // HDMI_EDID_BLOCK_SIZE = 128
        //
        // hdmi_read_edid()

        //if (buf[0x7e] != 0) {
        //    hdmi_read_edid(hdmi, 1, buf + HDMI_EDID_BLOCK_SIZE);
        //    edid_size += HDMI_EDID_BLOCK_SIZE;
        //}

        // return edid_size
        todo!();
    }

    // TODO - timeout
    // hdmi_read_edid()
    fn hdmi_read_edid(&mut self, block: usize) {
        // 0x6C6C6C6C
        //let val = unsafe { core::ptr::read_volatile((0x01EE_0000 + 0x7E0C) as *const
        // u32) }; panic!("0x7E0C = 0x{:X}", val);

        //let val = unsafe { core::ptr::read_volatile((0x01EE_0000 + 0x7E04) as *const
        // u32) }; panic!("0x7E04 = 0x{:X}", val);

        // Set ddc i2c clk which devided from ddc_clk to 100khz
        self.hdmi
            .r7e0c
            .modify(Rx7e0c::I2cmSsSclHcnt0Addr::Field::new(I2C_CLK_HIGH as _).unwrap());
        self.hdmi
            .r7e0c
            .modify(Rx7e0c::I2cmSsSclLcnt0Addr::Field::new(I2C_CLK_LOW as _).unwrap());

        // 0x7F
        unsafe { core::ptr::write_volatile((0x01EE_0000 + 0x7E0E) as *mut u8, 0xFE) };
        let val = unsafe { core::ptr::read_volatile((0x01EE_0000 + 0x7E0E) as *const u8) };
        //panic!("0x7E0E = 0x{:X}", val);

        let val = unsafe { core::ptr::read_volatile((0x01EE_0000 + 0x7E0C) as *const u32) };
        panic!("0x7E0C = 0x{:X}", val);

        let val = self
            .hdmi
            .r7e04
            .get_field(Rx7e04::I2cmDiv::Read)
            .unwrap()
            .val();
        self.hdmi
            .r7e04
            .modify(Rx7e04::I2cmDiv::Field::new(val | 0).unwrap());

        todo!()
    }
}
