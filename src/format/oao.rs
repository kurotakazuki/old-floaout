use crate::format::blow::Blower;

/// Details of the Floaout file.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Floaout {
    pub version: u8,
    pub length: u8,
    pub width: u8,
    pub height: u8,
    pub bubbles: u16,
    pub blocks: u64,
    pub sampling_rate: u32,
    pub bits_per_sample: u16,
    pub title_size: u8,
    pub title: String,
    pub artist_size: u8,
    pub artist: String,
    pub lyrics_size: u16,
    pub lyrics: String,
    pub image_code: u8,
    pub image_size: u32
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
            title_size: blower.title_size,
            title: blower.title,
            artist_size: blower.artist_size,
            artist: blower.artist,
            lyrics_size: blower.lyrics_size,
            lyrics: blower.lyrics,
            image_code: blower.image_code,
            image_size: blower.image_size
        }
    }
}