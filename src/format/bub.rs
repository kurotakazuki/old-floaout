/// Details of the Bubble file.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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
    pub name_size: u8
}