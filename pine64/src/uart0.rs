//! UART0
//!
//! Synopsys DesignWare ABP UART
//!
//! Size: 1K
//! Rx: PB9, PF4
//! Tx: PB8, PF2

use crate::uart_common::{
    NotConfigured, Receive, ReceiveRegisterBlock, RegisterBlock, Transmit, TransmitRegisterBlock,
    UartMode,
};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub const PADDR: usize = 0x01C2_8000;

pub struct UART0<RxTx> {
    _marker: PhantomData<RxTx>,
}

unsafe impl<RxTx: UartMode> Send for UART0<RxTx> {}

impl<RxTx: UartMode> UART0<RxTx> {
    pub unsafe fn from_paddr() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl UART0<NotConfigured> {
    pub fn as_ptr(&self) -> *const RegisterBlock {
        PADDR as *const _
    }

    pub const unsafe fn ptr() -> *const RegisterBlock {
        PADDR as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut RegisterBlock {
        PADDR as *mut _
    }

    pub const unsafe fn mut_ptr() -> *mut RegisterBlock {
        PADDR as *mut _
    }
}

impl Deref for UART0<NotConfigured> {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for UART0<NotConfigured> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

impl UART0<Receive> {
    pub fn as_ptr(&self) -> *const ReceiveRegisterBlock {
        PADDR as *const _
    }

    pub const unsafe fn ptr() -> *const ReceiveRegisterBlock {
        PADDR as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut ReceiveRegisterBlock {
        PADDR as *mut _
    }

    pub const unsafe fn mut_ptr() -> *mut ReceiveRegisterBlock {
        PADDR as *mut _
    }
}

impl Deref for UART0<Receive> {
    type Target = ReceiveRegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for UART0<Receive> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

impl UART0<Transmit> {
    pub fn as_ptr(&self) -> *const TransmitRegisterBlock {
        PADDR as *const _
    }

    pub const unsafe fn ptr() -> *const TransmitRegisterBlock {
        PADDR as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut TransmitRegisterBlock {
        PADDR as *mut _
    }

    pub const unsafe fn mut_ptr() -> *mut TransmitRegisterBlock {
        PADDR as *mut _
    }
}

impl Deref for UART0<Transmit> {
    type Target = TransmitRegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for UART0<Transmit> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
