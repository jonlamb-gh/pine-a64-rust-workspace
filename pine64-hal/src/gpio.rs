//! General Purpose Input / Output
//!
//! Abstracted over the PIO (CPUx-PORT port controller)
//!
//! Port B (PB): 10 input/output port
//! Port C (PC): 17 input/output port
//! Port D (PD): 25 input/output port
//! Port E (PE): 18 input/output port
//! Port F (PF):  7 input/output port
//! Port G (PG): 14 input/output port
//! Port H (PH): 12 input/output port
//!
//! PxY_Select variants mapped to alt functions:
//!   * 000 (U0): input
//!   * 001 (U1): output
//!   * 010 (U2): AF0
//!   * 011 (U3): AF1
//!   * 100 (U4): AF2
//!   * 101 (U5): AF3
//!   * 110 (U6): AF4
//!   * 111 (U7): disabled

use crate::ccu::Ccu;
use crate::hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
use crate::pac::ccu::BusClockGating2;
use crate::pac::pio::{Config0, Config1, Config2, Data, Driv0, Driv1, Pull0, Pull1, PIO};
use core::convert::Infallible;
use core::marker::PhantomData;

pub trait GpioExt {
    type Parts;

    fn split(self, ccu: &mut Ccu) -> Self::Parts;
}

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;

/// Input mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Alternate function mode
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Disabled mode
pub struct Disabled;

// See sun50i-a64.dtsi for drive strength values
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DriveStrength {
    L0_10mA,
    L1_20mA,
    L2_30mA,
    L3_40mA,
}

pub struct Gpio {
    pub pb: PortB,
    pub pc: PortC,
    pub pd: PortD,
}

impl GpioExt for PIO {
    type Parts = Gpio;

    fn split(self, ccu: &mut Ccu) -> Self::Parts {
        ccu.bcg2.enr().modify(BusClockGating2::Pio::Set);
        Gpio {
            pb: PortB::_new(),
            pc: PortC::_new(),
            pd: PortD::_new(),
        }
    }
}

pub struct PortB {
    pub pb0: PB0<Disabled>,
    pub pb1: PB1<Disabled>,
    pub pb2: PB2<Disabled>,
    pub pb3: PB3<Disabled>,
    pub pb4: PB4<Disabled>,
    pub pb5: PB5<Disabled>,
    pub pb6: PB6<Disabled>,
    pub pb7: PB7<Disabled>,
    pub pb8: PB8<Disabled>,
    pub pb9: PB9<Disabled>,
}

impl PortB {
    fn _new() -> Self {
        PortB {
            pb0: PB0 { _mode: PhantomData },
            pb1: PB1 { _mode: PhantomData },
            pb2: PB2 { _mode: PhantomData },
            pb3: PB3 { _mode: PhantomData },
            pb4: PB4 { _mode: PhantomData },
            pb5: PB5 { _mode: PhantomData },
            pb6: PB6 { _mode: PhantomData },
            pb7: PB7 { _mode: PhantomData },
            pb8: PB8 { _mode: PhantomData },
            pb9: PB9 { _mode: PhantomData },
        }
    }
}

pub struct PortC {
    pub pc0: PC0<Disabled>,
    pub pc1: PC1<Disabled>,
    pub pc2: PC2<Disabled>,
    pub pc3: PC3<Disabled>,
    pub pc4: PC4<Disabled>,
    pub pc5: PC5<Disabled>,
    pub pc6: PC6<Disabled>,
    pub pc7: PC7<Disabled>,
    pub pc8: PC8<Disabled>,
    pub pc9: PC9<Disabled>,
    pub pc10: PC10<Disabled>,
    pub pc11: PC11<Disabled>,
    pub pc12: PC12<Disabled>,
    pub pc13: PC13<Disabled>,
    pub pc14: PC14<Disabled>,
    pub pc15: PC15<Disabled>,
    pub pc16: PC16<Disabled>,
}

impl PortC {
    fn _new() -> Self {
        PortC {
            pc0: PC0 { _mode: PhantomData },
            pc1: PC1 { _mode: PhantomData },
            pc2: PC2 { _mode: PhantomData },
            pc3: PC3 { _mode: PhantomData },
            pc4: PC4 { _mode: PhantomData },
            pc5: PC5 { _mode: PhantomData },
            pc6: PC6 { _mode: PhantomData },
            pc7: PC7 { _mode: PhantomData },
            pc8: PC8 { _mode: PhantomData },
            pc9: PC9 { _mode: PhantomData },
            pc10: PC10 { _mode: PhantomData },
            pc11: PC11 { _mode: PhantomData },
            pc12: PC12 { _mode: PhantomData },
            pc13: PC13 { _mode: PhantomData },
            pc14: PC14 { _mode: PhantomData },
            pc15: PC15 { _mode: PhantomData },
            pc16: PC16 { _mode: PhantomData },
        }
    }
}

pub struct PortD {
    pub pd0: PD0<Disabled>,
    pub pd1: PD1<Disabled>,
    pub pd2: PD2<Disabled>,
    pub pd3: PD3<Disabled>,
    pub pd4: PD4<Disabled>,
    pub pd5: PD5<Disabled>,
    pub pd6: PD6<Disabled>,
    pub pd7: PD7<Disabled>,
    pub pd8: PD8<Disabled>,
    pub pd9: PD9<Disabled>,
    pub pd10: PD10<Disabled>,
    pub pd11: PD11<Disabled>,
    pub pd12: PD12<Disabled>,
    pub pd13: PD13<Disabled>,
    pub pd14: PD14<Disabled>,
    pub pd15: PD15<Disabled>,
    pub pd16: PD16<Disabled>,
}

impl PortD {
    fn _new() -> Self {
        PortD {
            pd0: PD0 { _mode: PhantomData },
            pd1: PD1 { _mode: PhantomData },
            pd2: PD2 { _mode: PhantomData },
            pd3: PD3 { _mode: PhantomData },
            pd4: PD4 { _mode: PhantomData },
            pd5: PD5 { _mode: PhantomData },
            pd6: PD6 { _mode: PhantomData },
            pd7: PD7 { _mode: PhantomData },
            pd8: PD8 { _mode: PhantomData },
            pd9: PD9 { _mode: PhantomData },
            pd10: PD10 { _mode: PhantomData },
            pd11: PD11 { _mode: PhantomData },
            pd12: PD12 { _mode: PhantomData },
            pd13: PD13 { _mode: PhantomData },
            pd14: PD14 { _mode: PhantomData },
            pd15: PD15 { _mode: PhantomData },
            pd16: PD16 { _mode: PhantomData },
        }
    }
}

macro_rules! gpio_pins {
    (
        // struct field name (r), register type (t)
        $CFGr:ident, $CFGt:ident,
        $DATAr:ident,
        $DRIVr:ident, $DRIVt:ident,
        $PULLr:ident, $PULLt:ident,
        [$($PXi:ident: ($pxi:ident, $px_field:ident, $MODE:ty),)+]
        ) => {
        $(
        pub struct $PXi<MODE> {
            _mode: PhantomData<MODE>,
        }

        impl<MODE> $PXi<MODE> {
            pub fn set_drive_strength(&mut self, level: DriveStrength) {
                match level {
                    DriveStrength::L0_10mA =>
                        unsafe { (*PIO::mut_ptr()).$DRIVr.modify($DRIVt::$px_field::Level0) },
                    DriveStrength::L1_20mA =>
                        unsafe { (*PIO::mut_ptr()).$DRIVr.modify($DRIVt::$px_field::Level1) },
                    DriveStrength::L2_30mA =>
                        unsafe { (*PIO::mut_ptr()).$DRIVr.modify($DRIVt::$px_field::Level2) },
                    DriveStrength::L3_40mA =>
                        unsafe { (*PIO::mut_ptr()).$DRIVr.modify($DRIVt::$px_field::Level3) },
                }
            }
        }

        impl<MODE> OutputPin for $PXi<Output<MODE>> {
            type Error = Infallible;

            fn set_high(&mut self) -> Result<(), Self::Error> {
                Ok(unsafe { (*PIO::mut_ptr()).$DATAr.modify(Data::$px_field::Set) })
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                Ok(unsafe { (*PIO::mut_ptr()).$DATAr.modify(Data::$px_field::Clear) })
            }
        }

        impl<MODE> InputPin for $PXi<Input<MODE>> {
            type Error = Infallible;

            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|b| !b)
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (*PIO::ptr()).$DATAr.is_set(Data::$px_field::Read) } != true)
            }
        }

        impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|b| !b)
            }

            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { (*PIO::ptr()).$DATAr.is_set(Data::$px_field::Read) } != true)
            }
        }

        impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}

        impl<MODE> $PXi<MODE> {
            #[inline]
            pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Input) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_pull_down_input(self) -> $PXi<Input<PullDown>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Input) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::PullDown) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Input) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::PullUp) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Output) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_alternate_af0(self) -> $PXi<Alternate<AF0>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Af0) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_alternate_af1(self) -> $PXi<Alternate<AF1>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Af1) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_alternate_af2(self) -> $PXi<Alternate<AF2>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Af2) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_alternate_af3(self) -> $PXi<Alternate<AF3>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Af3) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }

            #[inline]
            pub fn into_alternate_af4(self) -> $PXi<Alternate<AF4>> {
                unsafe { (*PIO::mut_ptr()).$CFGr.modify($CFGt::$px_field::Af4) };
                unsafe { (*PIO::mut_ptr()).$PULLr.modify($PULLt::$px_field::Disabled) };
                $PXi { _mode: PhantomData }
            }
        }
        )+
    }
}

gpio_pins!(
    pb_cfg0,
    Config0,
    pb_data,
    pb_driv0,
    Driv0,
    pb_pull0,
    Pull0,
    [
        PB0: (pb0, Pin0, Disabled),
        PB1: (pb1, Pin1, Disabled),
        PB2: (pb2, Pin2, Disabled),
        PB3: (pb3, Pin3, Disabled),
        PB4: (pb4, Pin4, Disabled),
        PB5: (pb5, Pin5, Disabled),
        PB6: (pb6, Pin6, Disabled),
        PB7: (pb7, Pin7, Disabled),
    ]
);

gpio_pins!(
    pb_cfg1,
    Config1,
    pb_data,
    pb_driv0,
    Driv0,
    pb_pull0,
    Pull0,
    [PB8: (pb8, Pin8, Disabled), PB9: (pb9, Pin9, Disabled),]
);

gpio_pins!(
    pc_cfg0,
    Config0,
    pc_data,
    pc_driv0,
    Driv0,
    pc_pull0,
    Pull0,
    [
        PC0: (pc0, Pin0, Disabled),
        PC1: (pc1, Pin1, Disabled),
        PC2: (pc2, Pin2, Disabled),
        PC3: (pc3, Pin3, Disabled),
        PC4: (pc4, Pin4, Disabled),
        PC5: (pc5, Pin5, Disabled),
        PC6: (pc6, Pin6, Disabled),
        PC7: (pc7, Pin7, Disabled),
    ]
);

gpio_pins!(
    pc_cfg1,
    Config1,
    pc_data,
    pc_driv0,
    Driv0,
    pc_pull0,
    Pull0,
    [
        PC8: (pc8, Pin8, Disabled),
        PC9: (pc9, Pin9, Disabled),
        PC10: (pc10, Pin10, Disabled),
        PC11: (pc11, Pin11, Disabled),
        PC12: (pc12, Pin12, Disabled),
        PC13: (pc13, Pin13, Disabled),
        PC14: (pc14, Pin14, Disabled),
        PC15: (pc15, Pin15, Disabled),
    ]
);

gpio_pins!(
    pc_cfg2,
    Config2,
    pc_data,
    pc_driv0,
    Driv1,
    pc_pull1,
    Pull1,
    [PC16: (pc16, Pin16, Disabled),]
);

gpio_pins!(
    pd_cfg0,
    Config0,
    pd_data,
    pd_driv0,
    Driv0,
    pd_pull0,
    Pull0,
    [
        PD0: (pd0, Pin0, Disabled),
        PD1: (pd1, Pin1, Disabled),
        PD2: (pd2, Pin2, Disabled),
        PD3: (pd3, Pin3, Disabled),
        PD4: (pd4, Pin4, Disabled),
        PD5: (pd5, Pin5, Disabled),
        PD6: (pd6, Pin6, Disabled),
        PD7: (pd7, Pin7, Disabled),
    ]
);

gpio_pins!(
    pd_cfg1,
    Config1,
    pd_data,
    pd_driv0,
    Driv0,
    pd_pull0,
    Pull0,
    [
        PD8: (pd8, Pin8, Disabled),
        PD9: (pd9, Pin9, Disabled),
        PD10: (pd10, Pin10, Disabled),
        PD11: (pd11, Pin11, Disabled),
        PD12: (pd12, Pin12, Disabled),
        PD13: (pd13, Pin13, Disabled),
        PD14: (pd14, Pin14, Disabled),
        PD15: (pd15, Pin15, Disabled),
    ]
);

gpio_pins!(
    pd_cfg2,
    Config2,
    pd_data,
    pd_driv0,
    Driv1,
    pd_pull1,
    Pull1,
    [PD16: (pd16, Pin16, Disabled),]
);
