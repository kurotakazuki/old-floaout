// Module of format

pub mod blow;
pub mod bub;
pub mod oao;
pub mod wav;

/// Detail of the Format.
/// This structure can be any of Blower, Bubble, Floaout and Wav.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Format {
    pub length: u8,
    pub width: u8,
    pub height: u8,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16
}

impl Format {
    pub fn channels(self) -> u16 {
        (1 << self.length) * (1 << self.width) * (1 << self.height)
    }
}