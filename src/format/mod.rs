// Module of format

pub mod blow;
pub mod bub;
pub mod oao;
pub mod wav;

/// Detail of the Format.
/// This structure can be any of Blower, Bubble, Floaout and Wav.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Format {
    pub channels: u16,
    pub sampling_rate: u32,
    pub bits_per_sample: u16,
    pub blocks: u64
}