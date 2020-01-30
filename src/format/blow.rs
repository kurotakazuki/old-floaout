/// Details of the Blower file.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Blower {
    pub version: u8,
    pub length: u8,
    pub width: u8,
    pub height: u8,
    pub bubbles: u16,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubblesInBlower {
    pub name_size: u8,
    pub name: String,
    pub times: u32,
    pub range: Vec<(u64, u64)>
}