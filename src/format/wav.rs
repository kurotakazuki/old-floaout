//! Structures related to `Wav`
//! 
//! Wav is a file which is a standard audio format.

use crate::format::Sample;
use std::fmt;

/// Details of the Wav file.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Wav {
    // Riff Chunk
    /// Riff Size is (file size - 8).
    pub riff_size: u32,
    // Format Chunk
    /// Format Size
    pub format_size: u32,
    /// Format Tag
    pub format_tag: u16,
    /// Channels
    pub channels: u16,
    /// Sampling Rate
    pub sampling_rate: u32,
    /// Data Rate
    pub data_rate: u32,
    /// Data Block Size
    pub data_block_size: u16,
    /// Bits Per Sample 
    pub bits_per_sample: u16,
    // Data Chunk
    /// Data Size
    pub data_size: u32,
    // Other Chunk
    /// Other Size is not exsists in a file.
    /// However, this will help erasing other chunk.
    pub other_size: u32
}

impl Wav {
    pub fn from_bits_per_sample(bits_per_sample: u16) -> Self {
        Self {
            bits_per_sample,
            ..Default::default()
        }
    }
}

impl fmt::Display for Wav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "\n File Size( - 8 ): {} Bytes\nFormat Chunk Size: {} Bytes\n      Format Tag: {}\n         Channels: {} Channels\n    Sampling Rate: {} Hz\n        Data Rate: {} kbps\n  Data Block Size: {} Bytes\n  Bits Per Sample: {} Bits\n   Wave Data Size: {} Bytes\n Other Chunk Size: {} Bytes\n",
            self.riff_size,
            self.format_size,
            self.format_tag,
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

/// Block of Wav
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WavBlock(pub Sample);

impl From<f32> for WavBlock {
    fn from(sample: f32) -> Self {
        WavBlock(Sample::Float32(sample))
    }
}

impl Into<f32> for WavBlock {
    fn into(self) -> f32 {
        (self.0).into()
    }
}

impl From<f64> for WavBlock {
    fn from(sample: f64) -> Self {
        WavBlock(Sample::Float64(sample))
    }
}

impl Into<f64> for WavBlock {
    fn into(self) -> f64 {
        (self.0).into()
    }
}

impl From<Sample> for WavBlock {
    fn from(sample: Sample) -> Self {
        WavBlock(sample)
    }
}

impl Into<Sample> for WavBlock {
    fn into(self) -> Sample {
        self.0
    }
}

/// Blocks of Wav
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WavBlocks(pub Box<[WavBlock]>);

impl From<Box<[WavBlock]>> for WavBlocks {
    fn from(buf: Box<[WavBlock]>) -> Self {
        WavBlocks(buf)
    }
}

impl Into<Box<[WavBlock]>> for WavBlocks {
    fn into(self) -> Box<[WavBlock]> {
        self.0
    }
}