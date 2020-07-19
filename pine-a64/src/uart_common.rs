//! Synopsys DesignWare ABP UART

use static_assertions::{assert_eq_align, assert_eq_size, const_assert_eq};

register! {
    ReceiveHolding,
    u32,
    RO,
    Fields [
        Data WIDTH(U8) OFFSET(U0),
    ]
}

register! {
    TransmitHolding,
    u32,
    WO,
    Fields [
        Data WIDTH(U8) OFFSET(U0),
    ]
}

register! {
    DivisorLatchLow,
    u32,
    RW,
    Fields [
        Lsb WIDTH(U8) OFFSET(U0),
    ]
}

register! {
    DivisorLatchHigh,
    u32,
    RW,
    Fields [
        Msb WIDTH(U8) OFFSET(U0),
    ]
}

register! {
    IntEnable,
    u32,
    RW,
    Fields [
        Erbfi WIDTH(U1) OFFSET(U0),
        Etbei WIDTH(U1) OFFSET(U1),
        Elsi WIDTH(U1) OFFSET(U2),
        Edssi WIDTH(U1) OFFSET(U3),
        Ptime WIDTH(U1) OFFSET(U7),
    ]
}

register! {
    IntStatus,
    u32,
    RO,
    Fields [
        PendingInt WIDTH(U4) OFFSET(U0) [
            ModemStatus = U0,
            NotPending = U1,
            ThrEmpty = U2,
            Rx = U4,
            RxLineStatus = U6,
            Busy = U7,
            CharTimeout = U12
        ]
        FifoEnabled WIDTH(U2) OFFSET(U6) [
            Disabled = U0,
            Enabled = U3
        ],
    ]
}

register! {
    FifoControl,
    u32,
    WO,
    Fields [
        FifoEnable WIDTH(U1) OFFSET(U0),
        RxFifoReset WIDTH(U1) OFFSET(U1),
        TxFifoReset WIDTH(U1) OFFSET(U2),
        DmaMode WIDTH(U1) OFFSET(U3) [
            Mode0 = U0,
            Mode1 = U1
        ]
        TxEmptryTrigger WIDTH(U2) OFFSET(U4) [
            OneChar = U0,
            QuarterFull = U1,
            HalfFull = U2,
            TwoFromFull = U3
        ]
        RxTrigger WIDTH(U2) OFFSET(U6) [
            FifoEmpty = U0,
            TwoChars = U1,
            QuarterFull = U2,
            HalfFull = U3
        ]
    ]
}

register! {
    LineControl,
    u32,
    RW,
    Fields [
        DataLength WIDTH(U2) OFFSET(U0) [
            FiveBits = U0,
            SixBits = U1,
            SevenBits = U2,
            EightBits = U3
        ]
        StopBits WIDTH(U1) OFFSET(U2) [
            One = U0,
            OneDotFive = U1
        ]
        ParityEnable WIDTH(U1) OFFSET(U3),
        EventParitySelect WIDTH(U2) OFFSET(U4) [
            Odd = U0,
            Event = U1,
            ReverseLCR = U2
        ]
        BreakControl WIDTH(U1) OFFSET(U6),
        DivisorLatchAccess WIDTH(U1) OFFSET(U7)
    ]
}

register! {
    ModemControl,
    u32,
    RW,
    Fields [
        DataTerminalReady WIDTH(U1) OFFSET(U0) [
            DtrDeAsserted = U0,
            DtrAsserted = U1
        ]
        RequestToSend WIDTH(U1) OFFSET(U1) [
            RtsDeAsserted = U0,
            RtsAsserted = U1
        ]
        LoopBackMode WIDTH(U1) OFFSET(U4),
        AutoFlowControl WIDTH(U1) OFFSET(U5),
        SirModeEnable WIDTH(U1) OFFSET(U6)
    ]
}

register! {
    LineStatus,
    u32,
    RO,
    Fields [
        DataReady WIDTH(U1) OFFSET(U0),
        OverrunError WIDTH(U1) OFFSET(U1),
        ParityError WIDTH(U1) OFFSET(U2),
        FramingError WIDTH(U1) OFFSET(U3),
        BreakInterrupt WIDTH(U1) OFFSET(U4),
        ThrEmpty WIDTH(U1) OFFSET(U5),
        TxEmpty WIDTH(U1) OFFSET(U6),
        RxFifoError WIDTH(U1) OFFSET(U7),
    ]
}

register! {
    ModemStatus,
    u32,
    RO,
    Fields [
        DeltaClearToSend WIDTH(U1) OFFSET(U0),
        DeltaDataSetReady WIDTH(U1) OFFSET(U1),
        TrailingEdgeRingInd WIDTH(U1) OFFSET(U2),
        DeltaDataCarDetect WIDTH(U1) OFFSET(U3),
        LineStateOfClearToSend WIDTH(U1) OFFSET(U4),
        LineStateOfDataSetReady WIDTH(U1) OFFSET(U5),
        LineStateOfRingInd WIDTH(U1) OFFSET(U6),
        LineStateOfDataCarDetect WIDTH(U1) OFFSET(U7),
    ]
}

register! {
    Scratch,
    u32,
    RW,
    Fields [
        Data WIDTH(U8) OFFSET(U0),
    ]
}

register! {
    Status,
    u32,
    RO,
    Fields [
        Busy WIDTH(U1) OFFSET(U0),
        TxFifoNotFull WIDTH(U1) OFFSET(U1),
        TxFifoEmpty WIDTH(U1) OFFSET(U2),
        RxFifoNotEmpty WIDTH(U1) OFFSET(U3),
        RxFifoFull WIDTH(U1) OFFSET(U4),
    ]
}

register! {
    TransmitFifoLevel,
    u32,
    RO,
    Fields [
        Count WIDTH(U7) OFFSET(U0),
    ]
}

register! {
    ReceiveFifoLevel,
    u32,
    RO,
    Fields [
        Count WIDTH(U7) OFFSET(U0),
    ]
}

const_assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x88);
assert_eq_align!(RegisterBlock, ReceiveRegisterBlock);
assert_eq_align!(RegisterBlock, TransmitRegisterBlock);
assert_eq_size!(RegisterBlock, ReceiveRegisterBlock);
assert_eq_size!(RegisterBlock, TransmitRegisterBlock);

#[repr(C)]
pub struct RegisterBlock {
    pub dll: DivisorLatchLow::Register,   // 0x00
    pub dlh: DivisorLatchHigh::Register,  // 0x04
    __reserved_1: u32,                    // 0x08
    pub lcr: LineControl::Register,       // 0x0C
    pub mcr: ModemControl::Register,      // 0x10
    pub lsr: LineStatus::Register,        // 0x14
    pub msr: ModemStatus::Register,       // 0x18
    pub spr: Scratch::Register,           // 0x1C
    __reserved_2: [u32; 23],              // 0x20
    pub sr: Status::Register,             // 0x7C
    pub tfl: TransmitFifoLevel::Register, // 0x80
    pub rfl: ReceiveFifoLevel::Register,  // 0x84
}

#[repr(C)]
pub struct ReceiveRegisterBlock {
    pub rhr: ReceiveHolding::Register,    // 0x00
    pub ier: IntEnable::Register,         // 0x04
    pub isr: IntStatus::Register,         // 0x08
    pub lcr: LineControl::Register,       // 0x0C
    pub mcr: ModemControl::Register,      // 0x10
    pub lsr: LineStatus::Register,        // 0x14
    pub msr: ModemStatus::Register,       // 0x18
    pub spr: Scratch::Register,           // 0x1C
    __reserved_0: [u32; 23],              // 0x20
    pub sr: Status::Register,             // 0x7C
    pub tfl: TransmitFifoLevel::Register, // 0x80
    pub rfl: ReceiveFifoLevel::Register,  // 0x84
}

#[repr(C)]
pub struct TransmitRegisterBlock {
    pub thr: TransmitHolding::Register,   // 0x00
    pub ier: IntEnable::Register,         // 0x04
    pub fcr: FifoControl::Register,       // 0x08
    pub lcr: LineControl::Register,       // 0x0C
    pub mcr: ModemControl::Register,      // 0x10
    pub lsr: LineStatus::Register,        // 0x14
    pub msr: ModemStatus::Register,       // 0x18
    pub spr: Scratch::Register,           // 0x1C
    __reserved_0: [u32; 23],              // 0x20
    pub sr: Status::Register,             // 0x7C
    pub tfl: TransmitFifoLevel::Register, // 0x80
    pub rfl: ReceiveFifoLevel::Register,  // 0x84
}

pub struct NotConfigured;
pub struct Receive;
pub struct Transmit;

pub trait UartMode: private::Sealed {}
impl UartMode for NotConfigured {}
impl UartMode for Receive {}
impl UartMode for Transmit {}

pub(crate) mod private {
    pub trait Sealed {}

    impl Sealed for super::NotConfigured {}
    impl Sealed for super::Receive {}
    impl Sealed for super::Transmit {}
}
