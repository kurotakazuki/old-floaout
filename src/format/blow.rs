//! Structures related to `Blower`

/// Details of the Blower file.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Blower {
    // Blower
    pub version: u8,
    // Bubble field
    pub length: u8,
    pub width: u8,
    pub height: u8,
    // Format
    pub bubbles: u16,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16
}

#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleInBlower {
    // Name of Bubble
    pub name_size: u8,
    pub name: String,
    // Number of ranges in Bubble.
    pub times: u32,
    // Ranges of Bubble.
    pub ranges: Vec<(u64, u64)>
}

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