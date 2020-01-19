// Module of format

pub mod blow;
pub mod bub;
pub mod oao;
pub mod wav;

use crate::format::wav::Wav;
use std::convert::From;

/// Detail of the Format.
/// This structure can be any of Blower, Bubble, Floaout and Wav.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Format {
    pub channels: u16,
    pub sampling_rate: u32,
    pub bits_per_sample: u16,
    pub blocks: u64
}

impl From<Wav> for Format {
    fn from(value: Wav) -> Self {
        Self {
            channels: value.channels,
            sampling_rate: value.sampling_rate,
            bits_per_sample: value.bits_per_sample,
            blocks: value.data_size as u64 / (value.bits_per_sample / 8) as u64 / value.channels as u64
        }
    }
}