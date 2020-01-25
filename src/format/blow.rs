use crate::format::Format;

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
    pub bits_per_sample: u16,
    pub title_size: u8,
    pub artist_size: u8,
    pub lyrics_size: u16,
    pub image_code: u8,
    pub image_size: u32
}

impl From<Format> for Blower {
    fn from(format: Format) -> Self {
        Self {
            blocks: format.blocks,
            sampling_rate: format.sampling_rate,
            bits_per_sample: format.bits_per_sample,
            ..Default::default()
        }
    }
}

impl Into<Format> for Blower {
    fn into(self) -> Format {
        Format {
            channels: 1,
            blocks: self.blocks,
            sampling_rate: self.sampling_rate,
            bits_per_sample: self.bits_per_sample
        }
    }
}