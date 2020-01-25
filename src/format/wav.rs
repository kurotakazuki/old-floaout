use crate::format::Format;
use std::convert::TryFrom;
use std::fmt;

/// Details of the Wav file.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Wav {
    // Riff Chunk
    pub riff_size: u32,
    // Format Chunk
    pub format_size: u32,
    pub format_code: u16,
    pub channels: u16,
    pub sampling_rate: u32,
    pub data_rate: u32,
    pub data_block_size: u16,
    pub bits_per_sample: u16,
    // Data Chunk
    pub data_size: u32,
    // Other Chunk
    pub other_size: u32
}

impl fmt::Display for Wav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "\n File Size( - 8 ): {} Bytes\nFormat Chunk Size: {} Bytes\n      Format Code: {}\n         Channels: {} Channels\n    Sampling Rate: {} Hz\n        Data Rate: {} kbps\n  Data Block Size: {} Bytes\n  Bits Per Sample: {} Bits\n   Wave Data Size: {} Bytes\n Other Chunk Size: {} Bytes\n",
            self.riff_size,
            self.format_size,
            self.format_code,
            self.channels,
            self.sampling_rate,
            self.data_rate,
            self.data_block_size,
            self.bits_per_sample,
            self.data_size,
            self.other_size
        )
    }
}

impl TryFrom<Format> for Wav {
    type Error = &'static str;

    fn try_from(value: Format) -> Result<Self, Self::Error> {
        let bytes_per_sample = value.bits_per_sample / 8;
        let riff_size = (value.channels * bytes_per_sample) as u64 * value.blocks + 36;
        if riff_size > u32::max_value() as u64 {
            Err("Wav riff size only accepts value no more than the largest value of u32!")
        } else {
            Ok(
                Self {
                    riff_size: riff_size as u32,
                    format_size: 16,
                    format_code: 3,
                    channels: value.channels,
                    sampling_rate: value.sampling_rate,
                    data_rate: value.sampling_rate * (bytes_per_sample * value.channels) as u32,
                    data_block_size: bytes_per_sample * value.channels,
                    bits_per_sample: value.bits_per_sample,
                    data_size: (riff_size - 36) as u32,
                    ..Default::default()
                }
            )
        }
    }
}