//! Delay

// TODO
// - embd-hal trait and impl over real timers

use cortex_a::asm;

pub(crate) fn delay_ms(ms: usize) {
    delay_us(ms * 1000);
}

pub(crate) fn delay_us(us: usize) {
    for _ in 0..us {
        for _ in 0..(24 + 10) {
            asm::nop();
        }
    }
}
