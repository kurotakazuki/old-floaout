use crate::format::Format;

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

impl From<Format> for Bubble {
    fn from(format: Format) -> Self {
        Self {
            version: 0,
            length: 0,
            width: 0,
            height: 0,
            red: 0,
            green: 0,
            blue: 0,
            blocks: format.blocks,
            sampling_rate: format.sampling_rate,
            bits_per_sample: format.bits_per_sample,
            name_size: 0
        }
    }
}

impl Into<Format> for Bubble {
    fn into(self) -> Format {
        Format {
            channels: 1,
            blocks: self.blocks,
            sampling_rate: self.sampling_rate,
            bits_per_sample: self.bits_per_sample
        }
    }
}