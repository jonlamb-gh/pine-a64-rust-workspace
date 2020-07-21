use crate::pac::ccu::{BusClockGating2, BusClockGating3, BusSoftReset4, CCU};

pub trait CcuExt {
    fn constrain(self) -> Ccu;
}

impl CcuExt for CCU {
    fn constrain(self) -> Ccu {
        Ccu {
            bcg2: BCG2 { _0: () },
            bcg3: BCG3 { _0: () },
            bsr4: BSR4 { _0: () },
        }
    }
}

pub struct Ccu {
    pub bcg2: BCG2,
    pub bcg3: BCG3,
    pub bsr4: BSR4,
}

pub struct BCG2 {
    _0: (),
}

impl BCG2 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating2::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg2 }
    }
}

pub struct BCG3 {
    _0: (),
}

impl BCG3 {
    pub(crate) fn enr(&mut self) -> &mut BusClockGating3::Register {
        unsafe { &mut (*CCU::mut_ptr()).bcg3 }
    }
}

pub struct BSR4 {
    _0: (),
}

impl BSR4 {
    pub(crate) fn rstr(&mut self) -> &mut BusSoftReset4::Register {
        unsafe { &mut (*CCU::mut_ptr()).bsr4 }
    }
}
