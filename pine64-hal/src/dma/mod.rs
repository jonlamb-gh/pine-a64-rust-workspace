//! DMA

// https://github.com/stm32-rs/stm32f7xx-hal/blob/master/src/dma.rs
//
// https://github.com/torvalds/linux/blob/master/drivers/dma/sun6i-dma.c
//
// https://github.com/torvalds/linux/blob/master/drivers/dma/sun6i-dma.c#L1159

use crate::pac::{
    ccu::{BusClockGating0, BusSoftReset0},
    dma::{AutoGating, DMA},
};
use crate::{ccu::Ccu, state};
use core::{
    marker::PhantomData,
    sync::atomic::{self, Ordering},
};

pub mod descriptor;
pub use descriptor::Descriptor;

// Channel<state> , state::Enabled/Disabled
// type state doesn't make sense here, DMA is enabled when created in the
// split()
//
// TransferResources
// Transfer

pub trait DmaExt {
    type Parts;

    fn split(self, ccu: &mut Ccu) -> Self::Parts;
}

pub struct Dma {
    pub ch0: Channel<state::Disabled>,
    // TODO ch1..=ch7
}

pub struct Channel<State> {
    dma: DMA,
    chan_num: u8,
    _state: State,
}

impl DmaExt for DMA {
    type Parts = Dma;

    fn split(self, ccu: &mut Ccu) -> Self::Parts {
        ccu.bcg0.enr().modify(BusClockGating0::Dma::Set);
        ccu.bsr0.rstr().modify(BusSoftReset0::Dma::Clear);
        ccu.bsr0.rstr().modify(BusSoftReset0::Dma::Set);

        let mut dma = unsafe { DMA::from_paddr() };

        dma.auto_gating.modify(AutoGating::MasterClock::Enable);

        Dma {
            ch0: Channel {
                dma,
                chan_num: 0,
                _state: state::Disabled,
            },
        }
    }
}

impl Channel<state::Disabled> {
    pub fn enable(self) -> Channel<state::Enabled> {
        Channel {
            dma: self.dma,
            chan_num: self.chan_num,
            _state: state::Enabled,
        }
    }
}
