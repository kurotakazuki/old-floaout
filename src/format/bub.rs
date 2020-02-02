//! Structures related to `Bubble`

use crate::format::wav::Wav;
use std::convert::{TryFrom, TryInto};

/// Details of the Bubble file.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bubble {
    /// This is the number of `Bubble` version.
    pub version: u8,
    // Bubble field
    /// This is length of Bubble field.
    /// ```
    /// 2^length
    /// ```
    pub length: u8,
    /// This is width of Bubble field.
    /// ```
    /// 2^width
    /// ```
    pub width: u8,
    /// This is height of Bubble field.
    /// ```
    /// 2^height
    /// ```
    pub height: u8,
    // Color
    /// Red (0~255)
    pub red: u8,
    /// Green (0~255)
    pub green: u8,
    /// Blue (0~255)
    pub blue: u8,
    // Format
    /// This is number of block.
    pub blocks: u64,
    /// Sampling Rate
    pub sampling_rate: u32,
    /// Bits Per Sample
    pub bits_per_sample: u16,
    /// Bubble Name Size
    pub name_size: u8,
    /// Name of Bubble
    pub name: String,
    /// Overall of Bubble field
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
                    other_size: 0
                }
            )
        }
    }
}