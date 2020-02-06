//! Structures related to `Blower`

use crate::format::BubbleFieldSize;

/// Details of the Blower file.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Blower {
    /// This is the number of `Blower` version.
    pub version: u8,
    /// This includes length, width and height.
    pub bub_field_size: BubbleFieldSize,
    // Format
    /// Number of Bubble in Floaout
    pub bubbles: u16,
    /// This is number of block.
    pub blocks: u64,
    /// Sampling Rate
    pub sampling_rate: u32,
    /// Bits Per Sample
    pub bits_per_sample: u16
}

/// This structure contains data about Bubble in Blower.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleInBlower {
    /// Bubble Name Size
    pub name_size: u8,
    /// Name of Bubble
    pub name: String,
    /// Number of ranges in Bubble.
    pub times: u32,
    /// Ranges of Bubble.
    pub ranges: Vec<(u64, u64)>
}

/// This structure covers all 'BubbleInBlower' in Blower.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubblesInBlower(Vec<BubbleInBlower>);

impl From<Vec<BubbleInBlower>> for BubblesInBlower {
    fn from(value: Vec<BubbleInBlower>) -> Self {
        BubblesInBlower(value)
    }
}

impl Into<Vec<BubbleInBlower>> for BubblesInBlower {
    fn into(self) -> Vec<BubbleInBlower> {
        self.0 as Vec<BubbleInBlower>
    }
}

impl From<Blower> for Floaout {
    fn from(blower: Blower) -> Self {
        Self {
            version: blower.version,
            song_id: 0,
            bub_field_size: blower.bub_field_size,
            bubbles: blower.bubbles,
            blocks: blower.blocks,
            sampling_rate: blower.sampling_rate,
            bits_per_sample: blower.bits_per_sample,
        }
    }
}