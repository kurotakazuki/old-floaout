use crate::format::blow::Blower;

/// Details of the Floaout file.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Floaout {
    pub version: u8,
    pub song_id: u64,
    pub length: u8,
    pub width: u8,
    pub height: u8,
    pub bubbles: u16,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16
}

impl From<Blower> for Floaout {
    fn from(blower: Blower) -> Self {
        Self {
            version: blower.version,
            length: blower.length,
            width: blower.width,
            height: blower.height,
            bubbles: blower.bubbles,
            blocks: blower.blocks,
            sampling_rate: blower.sampling_rate,
            bits_per_sample: blower.bits_per_sample,
        }
    }
}