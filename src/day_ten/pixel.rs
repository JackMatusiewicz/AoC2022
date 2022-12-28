#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
#[repr(u8)]
pub enum PixelState {
    Off = 0,
    On = 1,
}
