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
- generate UART1-4 impls with a macro

https://github.com/torvalds/linux/blob/master/drivers/tty/serial/8250/8250_dw.c
