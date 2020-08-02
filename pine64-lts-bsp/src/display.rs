use crate::hal::ccu::Ccu;
use crate::hal::pac::tcon1::TCON1;

// hpd = 1
// hpd_delay = 500
// edid = 1
//
// overscan_x = 0
// overscan_y = 0
//
// x8r8g8b8

// HDMI reg defs in arch/arm/include/asm/arch-sunxi/display.h
// sunxi_hdmi_reg

pub struct HdmiDisplay<'a> {
    tcon1: TCON1,
    frame_buffer: &'a mut [u32],
}

impl<'a> HdmiDisplay<'a> {
    pub fn new(tcon1: TCON1, frame_buffer: &'a mut [u32], ccu: &mut Ccu) -> Self {
        // TODO - checks/etc

        HdmiDisplay {
            tcon1,
            frame_buffer,
        }
    }

    fn video_hw_init(&mut self) {
        // drivers/video/sunxi/sunxi_display.c
        // video_hw_init()

        // hdmi_present = (sunxi_hdmi_hpd_detect(hpd_delay) == 1)
        // ... if present ...
        // Fall back to EDID in case HPD failed
        //
        // Shut down when display was not found
        // sunxi_hdmi_shutdown()
        //
        // sunxi_has_hdmi() -> always true for this bsp

        // sunxi_display.fb_size =
        //   (mode->xres * mode->yres * 4 + 0xfff) & ~0xfff;

        //overscan_offset = (overscan_y * mode->xres + overscan_x) * 4;

        /* We want to keep the fb_base for simplefb page aligned, where as
         * the sunxi dma engines will happily accept an unaligned address. */
        // if (overscan_offset)
        //    sunxi_display.fb_size += 0x1000;

        // gd->fb_base = gd->bd->bi_dram[0].start +
        //      gd->bd->bi_dram[0].size - sunxi_display.fb_size;
        // sunxi_engines_init();

        /*
        fb_dma_addr = gd->fb_base - CONFIG_SYS_SDRAM_BASE;
        sunxi_display.fb_addr = gd->fb_base;
        if (overscan_offset) {
            fb_dma_addr += 0x1000 - (overscan_offset & 0xfff);
            sunxi_display.fb_addr += (overscan_offset + 0xfff) & ~0xfff;
            memset((void *)gd->fb_base, 0, sunxi_display.fb_size);
            flush_cache(gd->fb_base, sunxi_display.fb_size);
        }
        sunxi_mode_set(mode, fb_dma_addr);
        */

        /*
        graphic_device->frameAdrs = sunxi_display.fb_addr;
        graphic_device->gdfIndex = GDF_32BIT_X888RGB;
        graphic_device->gdfBytesPP = 4;
        graphic_device->winSizeX = mode->xres - 2 * overscan_x;
        graphic_device->winSizeY = mode->yres - 2 * overscan_y;
        graphic_device->plnSizeX = mode->xres * graphic_device->gdfBytesPP;
        */
    }

    fn hdmi_hpd_detect(hpd_delay: usize) {
        // sunxi_hdmi_hpd_detect()
        todo!()
    }
}
