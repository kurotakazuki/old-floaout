use crate::format::blow::Blower;

/// Details of the Floaout file.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Floaout {
    // Floaout
    pub version: u8,
    pub song_id: u64,
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

impl From<Blower> for Floaout {
    fn from(blower: Blower) -> Self {
        Self {
            version: blower.version,
            song_id: 0,
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