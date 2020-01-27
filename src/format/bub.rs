use crate::format::wav::Wav;
use std::convert::{TryFrom, TryInto};

/// Details of the Bubble file.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bubble {
    pub version: u8,
    pub length: u8,
    pub width: u8,
    pub height: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16,
    pub name_size: u8,
    pub name: String,
    pub overall: Vec<Vec<Vec<u8>>>
}

impl TryFrom<Wav> for Bubble {
    type Error = &'static str;

    fn try_from(wav: Wav) -> Result<Self, Self::Error> {
        if wav.channels == 1 {
            Ok(
                Self {
                    blocks: (wav.data_size / wav.data_block_size as u32) as u64,
                    sampling_rate: wav.sampling_rate,
                    bits_per_sample: wav.bits_per_sample,
                    ..Default::default()
                }
            )
        } else {
            Err("Bubble only accepts 1 channel.")
        }
    }
}

impl TryInto<Wav> for Bubble {
    type Error = &'static str;

    fn try_into(self) -> Result<Wav, Self::Error> {
        let bytes_per_sample = self.bits_per_sample / 8;
        let riff_size = 1 * bytes_per_sample as u64 * self.blocks + 36;
        if riff_size > u32::max_value() as u64 {
            Err("Wav riff size only accepts no more than the largest value of u32.")
        } else {
            Ok(
                Wav {
                    riff_size: riff_size as u32,
                    format_size: 16,
                    format_tag: 3,
                    channels: 1,
                    sampling_rate: self.sampling_rate,
                    data_rate: self.sampling_rate * (bytes_per_sample * 1) as u32,
                    data_block_size: bytes_per_sample * 1,
                    bits_per_sample: self.bits_per_sample,
                    data_size: (riff_size - 36) as u32,
                    ..Default::default()
                }
            )
        }
    }
}