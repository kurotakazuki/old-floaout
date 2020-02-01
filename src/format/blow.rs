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
    pub name: String,
    pub times: u32,
    pub range: Vec<(u64, u64)>
}