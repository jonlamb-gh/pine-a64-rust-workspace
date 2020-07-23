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

## U-boot

TODO

## Links

- [Pine A64 Specs](https://wiki.pine64.org/index.php?title=PINE_A64-LTS/SOPine#SoC_and_Memory_Specification)
- [PinePhone Specs](https://wiki.pine64.org/index.php?title=PinePhone#Specifications)
- [linux-sunxi A64](https://linux-sunxi.org/A64)
- [sun50i-a64-sopine-baseboard.dts](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64-sopine-baseboard.dts)
- [sun50i-a64-sopine.dtsi](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64-sopine.dtsi)
- [sun50i-a64.dtsi](https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64.dtsi)

## TODOs

- check svd2rust for the latest peripheral materialization patterns
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


Stuff for the PinePhone BSP crate
- PinePhone debug UART is UART0, PB8/PB9


boot process in
https://github.com/u-boot/u-boot/blob/master/board/sunxi/README.sunxi64

u-boot sunxi video drivers
https://github.com/u-boot/u-boot/tree/master/drivers/video/sunxi

arch/arm/include/asm/arch-sunxi/cpu_sun4i.h
CONFIG_MACH_SUN50I

A10 display pipeline docs
https://www.kernel.org/doc/Documentation/devicetree/bindings/display/sunxi/sun4i-drm.txt

framebuff-lcd in
https://github.com/torvalds/linux/blob/master/arch/arm64/boot/dts/allwinner/sun50i-a64.dtsi

video engine: sun50i-a64-video-engine
https://github.com/torvalds/linux/blob/master/drivers/staging/media/sunxi/cedrus/cedrus.c
https://github.com/torvalds/linux/blob/master/drivers/staging/media/sunxi/cedrus/cedrus_regs.h

sun50i-a64-display-engine
https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/sun4i/sun4i_drv.c

sun50i-a64-de2-mixer-0
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
