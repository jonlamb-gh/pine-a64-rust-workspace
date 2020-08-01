// TODO Some of this was copied from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

#![deny(warnings)]
#![no_std]
#![feature(global_asm)]
#![cfg_attr(feature = "panic-abort", feature(core_intrinsics))]

#[cfg(feature = "panic-abort")]
mod panic_abort;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    extern "Rust" {
        fn main() -> !;
    }

    main();
}

// Disable all cores except core 0, and then jump to reset()
global_asm!(include_str!("boot_cores.S"));
