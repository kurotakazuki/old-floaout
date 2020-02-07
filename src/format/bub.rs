//! Structures related to `Bubble`

use crate::format::{BubbleField, BubbleFieldSize, Color};
use crate::format::oao::Floaout;
use crate::format::wav::{Wav, WavBlock};
use std::convert::{TryFrom, TryInto};

/// Details of the Bubble file.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bubble {
    /// This is the number of `Bubble` version.
    pub version: u8,
    /// This includes length, width and height.
    pub bub_field_size: BubbleFieldSize,
    /// This includes red, green, blue.
    pub color: Color,
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
    pub overall: BubbleField
}

impl Bubble {
    /// This method casts Bubble field size and bits per sample to `Bubble`.
    /// 
    /// # Examples
    /// ```
    /// use floaout::format::bub::Bubble;
    /// 
    /// let bub = Bubble::from_bub_field_size_and_bits_per_sample((0u8, 0u8, 0u8).into(), 32);
    /// 
    /// assert_eq!(
    ///     bub,
    ///     Bubble {
    ///         bub_field_size: (0u8, 0u8, 0u8).into(),
    ///         bits_per_sample: 32,
    ///         ..Default::default()
    ///     }
    /// );
    /// ```
    pub fn from_bub_field_size_and_bits_per_sample(bub_field_size: BubbleFieldSize, bits_per_sample: u16) -> Self {
        Self {
            bub_field_size,
            bits_per_sample,
            ..Default::default()
        }
    }
}

impl From<Floaout> for Bubble {
    fn from(oao: Floaout) -> Self {
        Self {
            bub_field_size: oao.bub_field_size,
            blocks: oao.blocks,
            sampling_rate: oao.sampling_rate,
            bits_per_sample: oao.bits_per_sample,
            ..Default::default()
        }
    }
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

/// Block of Bubble
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BubbleBlock {
    /// Block of Wav
    pub wav_block: WavBlock,
    /// Bubble field
    pub bub_field: BubbleField
}

impl BubbleBlock {
    /// This method casts `WavBlock` and `BubbleField` to `Bubble`.
    /// 
    /// # Examples
    /// ```
    /// use floaout::format::BubbleField;
    /// use floaout::format::bub::{Bubble, BubbleBlock};
    /// use floaout::format::wav::WavBlock;
    /// 
    /// let bub_field: BubbleField = vec![vec![vec![0], vec![2]], vec![vec![1], vec![3]]].into();
    /// 
    /// let bub_block = BubbleBlock::from_wav_block_and_bub_field(1.0.into(), bub_field);
    /// ```
    pub fn from_wav_block_and_bub_field(wav_block: WavBlock, bub_field: BubbleField) -> Self {
        Self {
            wav_block,
            bub_field
        }
    }
}

/// Blocks of Bubble
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BubbleBlocks(pub Box<[BubbleBlock]>);

impl From<Box<[BubbleBlock]>> for BubbleBlocks {
    fn from(buf: Box<[BubbleBlock]>) -> Self {
        BubbleBlocks(buf)
    }
}

impl Into<Box<[BubbleBlock]>> for BubbleBlocks {
    fn into(self) -> Box<[BubbleBlock]> {
        self.0
    }
}