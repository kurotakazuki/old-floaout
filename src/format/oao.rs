//! Structures related to `Floaout`
//! 
//! Floaout is the forefront audio format that enables immersive sound which takes advantage of both channel-based and object-based system.

use crate::format::{BubbleFieldSize, Color};
use crate::format::bub::BubbleBlock;

/// Details of the Floaout file.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Floaout {
    /// This is the number of `Floaout` version.
    pub version: u8,
    /// It links to title, artist, lyrics, image, etc.
    /// This ID will save the file size and enhance versatility.
    ///  Developers also have the merit that they don't have to process title string and so on in the file and allow concentrate on audio processing.
    /// Song ID is 8 bytes data. It links to title, artist, lyrics, image, etc.
    /// This ID will save the file size and enhance versatility.
    /// Developers also have the merit that they don't have to process title string and so on in the file and allow concentrate on audio processing.
    /// If Song ID is "0x0000000000000000", this means no link.
    pub song_id: u64,
    /// This includes length, width and height.
    pub bub_field_size: BubbleFieldSize,
    // Format
    /// Number of Bubble in Floaout
    pub bubbles: u16,
    /// This is number of block.
    pub blocks: u64,
    /// Sampling Rate
    pub sampling_rate: u32,
    /// Bits Per Sample
    pub bits_per_sample: u16,
    /// Title Size
    pub title_size: u8,
    /// Title
    pub title: String,
    /// Artist Size
    pub artist_size: u8,
    /// Artist
    pub artist: String
}

/// This structure contains data about Bubble in Floaout.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleInFloaout {
    /// Bubble ID is 16 bytes data. It links to copyright.
    /// This ID will help artists to get royalty and easy to make remixs.
    /// If Bubble ID is "0", this means no link.
    pub bubble_id: u128,
    /// Bubble Name Size
    pub name_size: u8,
    /// Name of Bubble
    pub name: String,
    /// This includes red, green, blue.
    pub color: Color
}

/// This structure covers all 'BubbleInFloaout' in Floaout.
/// This block is Vec of Bubble block.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubblesInFloaout(pub Vec<BubbleInFloaout>);

impl From<Vec<BubbleInFloaout>> for BubblesInFloaout {
    fn from(value: Vec<BubbleInFloaout>) -> Self {
        BubblesInFloaout(value)
    }
}

impl Into<Vec<BubbleInFloaout>> for BubblesInFloaout {
    fn into(self) -> Vec<BubbleInFloaout> {
        self.0 as Vec<BubbleInFloaout>
    }
}

/// Block of Floaout
/// This block is Vec of Bubble block.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FloaoutBlock(pub Vec<BubbleBlock>);

impl From<Vec<BubbleBlock>> for FloaoutBlock {
    fn from(value: Vec<BubbleBlock>) -> Self {
        FloaoutBlock(value)
    }
}

impl Into<Vec<BubbleBlock>> for FloaoutBlock {
    fn into(self) -> Vec<BubbleBlock> {
        self.0 as Vec<BubbleBlock>
    }
}

/// Blocks of Floaout
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FloaoutBlocks(pub Box<[FloaoutBlock]>);

impl From<Box<[FloaoutBlock]>> for FloaoutBlocks {
    fn from(buf: Box<[FloaoutBlock]>) -> Self {
        FloaoutBlocks(buf)
    }
}

impl Into<Box<[FloaoutBlock]>> for FloaoutBlocks {
    fn into(self) -> Box<[FloaoutBlock]> {
        self.0
    }
}