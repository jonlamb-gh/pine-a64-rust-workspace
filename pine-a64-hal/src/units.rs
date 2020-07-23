/// Bits per second
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Bps(pub u32);

pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }
}
