# Pine A64 Rust Workspace

## Crates

TODO

## Building

`rustc 1.46.0-nightly (346aec9b0 2020-07-11)`

```rust
rustup target add aarch64-unknown-none

cargo build --release
```

Copy elf to binary:

```bash
cargo objcopy -- -O binary target/aarch64-unknown-none/release/<img> /dest/<img>.bin
```

```bash
cargo objcopy --release -- -O binary /dest/<img>.bin
```

## U-boot

TODO - track u-boot.cfg here

```bash
CONFIG_CMD_CACHE=y
CONFIG_ENV_FAT_DEVICE_AND_PART="0:auto"

ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- make pine64-lts_defconfig
ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- make
dd if=u-boot-sunxi-with-spl.bin of=/dev/sda bs=1k seek=8
```

```text
U-Boot SPL 2020.10-rc1-00148-g719f42190d-dirty (Aug 01 2020 - 07:59:39 -0700)
...

U-Boot 2020.10-rc1-00148-g719f42190d-dirty (Aug 01 2020 - 08:04:33 -0700) Allwinner Technology
...
```

Environment:

```bash
setenv imgname img.bin

setenv loadaddr 0x42000000 ?

# Make sure the caches are off for now
setenv bootimg 'tftp ${loadaddr} ${serverip}:${imgname}; dcache flush; dcache off; go ${loadaddr}'
```

## Links

- [Pine A64 Specs](https://wiki.pine64.org/index.php?title=PINE_A64-LTS/SOPine#SoC_and_Memory_Specification)
- [PinePhone Specs](https://wiki.pine64.org/index.php?title=PinePhone#Specifications)
- [linux-sunxi A64](https://linux-sunxi.org/A64)
- [sun50i-a64-sopine-baseboard.dts](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64-sopine-baseboard.dts)
- [sun50i-a64-sopine.dtsi](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64-sopine.dtsi)
- [sun50i-a64.dtsi](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64.dtsi)

## ...

- rename `pine-a64...` to `pine64...`
- check svd2rust for the latest peripheral materialization patterns
  * https://github.com/rust-embedded/cortex-m/commit/64dc07d286163bc0c666b7d7058107c3f688bb32
- interrupts and consts in the device crate
  * https://github.com/rust-embedded/cortex-m/pull/241
  * https://github.com/rust-embedded/cortex-m/pull/235
- update the boot crate to use `llvm_asm!`
- switch UART device to have aliased registers instead of multiple register blocks and type state
- add all the PIO registers/pins/etc
- gpio ExtiPin patterns
- generate UART1-4 device/reg impls with a macro
- CCU device for peripheral resets/etc
- use Infallible instead of Void
- BSP crates: pine-a64-lts and pinephone
- PR on bounded-regs for having a field named `Width` breaking things


Stuff for the PinePhone BSP crate
- PinePhone debug UART is UART0, PB8/PB9

this u-boot image works differently than upstream u-boot?
https://github.com/apritzel/pine64
https://github.com/apritzel/u-boot.git -b sunxi64-image-20180316
sun50i-a64-lpddr3_defconfig

https://github.com/longsleep/build-pine64-image/tree/master/blobs
https://github.com/armbian/build/tree/master/packages/blobs/sunxi/a64

this works:

```bash
spl from https://github.com/apritzel/u-boot.git, branch: sunxi64-image-20180316
sudo dd if=spl/sunxi-spl.bin of=/dev/sda bs=8k seek=1

u-boot from upstream
sudo dd if=u-boot.itb of=/dev/sda bs=8k seek=5
```

some configs from `u-boot.cfg` from `pine64-lts_defconfig`:

```text
#define CONFIG_SUNXI_GEN_SUN6I 1
#define CONFIG_MACH_SUN50I 1
#define CONFIG_MMC_SUNXI 1
#define CONFIG_SUNXI_DE2 1
#define CONFIG_SYS_CONFIG_NAME "sun50i"
#define CONFIG_SUN8I_EMAC 1
#define CONFIG_SUNXI_DRAM_MAX_SIZE 0xC0000000
#define CONFIG_ARCH_SUNXI 1
#define CONFIG_SUNXI_GPIO 1
#define CONFIG_CLK_SUN50I_A64 1
#define CONFIG_SUN6I_PRCM 1
#define CONFIG_CLK_SUNXI 1
#define CONFIG_PHY_SUN4I_USB 1
#define CONFIG_DEFAULT_DEVICE_TREE "sun50i-a64-pine64-lts"
#define CONFIG_DM_VIDEO 1
#define CONFIG_VIDEO_DT_SIMPLEFB 1
#define CONFIG_VIDEO_DE2 1
```
arch/arm/include/asm/arch-sunxi/cpu_sun4i.h

boot process in
https://github.com/u-boot/u-boot/blob/master/board/sunxi/README.sunxi64

u-boot sunxi video drivers
https://github.com/u-boot/u-boot/tree/master/drivers/video/sunxi

A10 display pipeline docs
https://www.kernel.org/doc/Documentation/devicetree/bindings/display/sunxi/sun4i-drm.txt

framebuff-lcd in
```text
tcon0: lcd-controller@0x01C0_C000, allwinner,sun50i-a64-tcon-lcd
tcon1: lcd-controller@0x01C0_D000, allwinner,sun50i-a64-tcon-tv

cpu_sun4i.h
SUNXI_DE2_BASE = 0x0100_0000

display2.h
SUNXI_DE2_BASE == struct de_clk
SUNXI_DE2_MUX0_BASE = SUNXI_DE2_BASE + 0x10_0000
SUNXI_DE2_MUX1_BASE = SUNXI_DE2_BASE + 0x20_0000

each mixer reg block is identical
SUNXI_DE2_MUX_GLB_REGS, struct de_glb

TODO check the CSC offsets u-boot vs linux seem off
SUNXI_DE2_MUX_DCSC_REGS = 0xb0000 = struct de_csc
in linux they use DE3_BLD_BASE 0x0800 as the offset?

also in linux, SUN50I vs SUN8I registers are at different addresses

TODO - are the mixer addresses relative to the DE block 0x0100_0000--0x013F_FFFF ?
mixer0: mixer@0x0010_0000 ,allwinner,sun50i-a64-de2-mixer-0
mixer1: mixer@0x0020_0000, allwinner,sun50i-a64-de2-mixer-1

video-codec@0x01C0_E000, allwinner,sun50i-a64-video-engine

hdmi: hdmi@0x01EE_0000, allwinner,sun50i-a64-dw-hdmi
hdmi_phy: hdmi-phy@01EF_0000, allwinner,sun50i-a64-hdmi-phy

ddc-i2c-bus == i2c3 ?

deinterlace: deinterlace@0x01e0_0000, allwinner,sun50i-a64-deinterlace
0x01E0_0000---0x01E1_FFFF
```
https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64.dtsi

video engine: sun50i-a64-video-engine
https://github.com/torvalds/linux/blob/master/drivers/staging/media/sunxi/cedrus/cedrus.c
https://github.com/torvalds/linux/blob/master/drivers/staging/media/sunxi/cedrus/cedrus_regs.h

sun50i-a64-display-engine
https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/sun4i/sun4i_drv.c

sun50i-a64-de2-mixer-0
https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/sun4i/sun8i_mixer.h
https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/sun4i/sun8i_mixer.c


https://github.com/rust-embedded/svd2rust/blob/master/src/generate/peripheral.rs
https://github.com/rust-embedded/svd2rust/blob/master/src/generate/generic.rs

https://github.com/michalsc/Emu68/tree/master/src/pine64
https://github.com/krjdev/rock64_bare-metal

https://docs.rs/embedded-hal/0.2.4/embedded_hal/
https://docs.rs/embedded-time/0.7.1/embedded_time/

https://github.com/torvalds/linux/blob/master/drivers/pinctrl/sunxi/pinctrl-sun50i-a64.c
https://github.com/torvalds/linux/blob/master/drivers/pinctrl/sunxi/pinctrl-sunxi.c
https://github.com/torvalds/linux/blob/master/drivers/pinctrl/sunxi/pinctrl-sunxi.h

https://github.com/torvalds/linux/blob/master/drivers/tty/serial/8250/8250_dw.c
https://github.com/RTEMS/rtems/blob/master/bsps/arm/altera-cyclone-v/include/bsp/alt_16550_uart.h
https://github.com/RTEMS/rtems/blob/master/bsps/arm/altera-cyclone-v/contrib/hwlib/src/hwmgr/alt_16550_uart.c

example bare metal UART
https://github.com/linux-sunxi/sunxi-tools/blob/master/uart0-helloworld-sdboot.c#L367

https://github.com/stm32-rs/stm32f1xx-hal
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/rcc.rs
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/gpio.rs

https://github.com/japaric/stm32f30x-hal/blob/master/src/serial.rs

https://www.freertos.org/Using-FreeRTOS-on-Cortex-A-Embedded-Processors.html
