/// Details of the Blower file.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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
    pub title: String,
    pub artist_size: u8,
    pub artist: String,
    pub lyrics_size: u16,
    pub lyrics: String,
    pub image_code: u8,
    pub image_size: u32
}