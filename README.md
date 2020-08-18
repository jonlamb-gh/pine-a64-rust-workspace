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

setenv loadaddr 0x42000000

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

using SPL from apritzel/u-boot.git works:

```bash
spl from https://github.com/apritzel/u-boot.git, branch: sunxi64-image-20180316
sudo dd if=spl/sunxi-spl.bin of=/dev/sda bs=8k seek=1

u-boot from upstream
sudo dd if=u-boot.itb of=/dev/sda bs=8k seek=5
```

https://github.com/stm32-rs/stm32f1xx-hal
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/rcc.rs
https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/gpio.rs

https://github.com/japaric/stm32f30x-hal/blob/master/src/serial.rs

https://github.com/stm32-rs/stm32f7xx-hal/blob/master/src/dma.rs

https://www.freertos.org/Using-FreeRTOS-on-Cortex-A-Embedded-Processors.html
