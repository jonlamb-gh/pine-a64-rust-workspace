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
- update the boot crate to use `llvm_asm!`
- interrupts and consts in the device crate
- switch UART device to have aliased registers instead of multiple register blocks and type state
- add all the PIO registers/pins/etc
- gpio ExtiPin patterns
- generate UART1-4 device/reg impls with a macro
- CCU device for peripheral resets/etc
- use Infallible instead of Void


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

https://github.com/stm32-rs/stm32f1xx-hal
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/rcc.rs
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/gpio.rs


