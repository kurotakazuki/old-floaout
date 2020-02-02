//! Structures related to `Floaout`
//! 
//! Floaout is the forefront audio format that enables immersive sound which takes advantage of both channel-based and object-based system.

use crate::format::blow::Blower;

/// Details of the Floaout file.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Floaout {
    /// This is the number of `Floaout` version.
    pub version: u8,
    /// It links to title, artist, lyrics, image, etc.
    /// This ID will save the file size and enhance versatility.
    ///  Developers also have the merit that they don't have to process title string and so on in the file and allow concentrate on audio processing.
    ///  If Song ID is "0x0000000000000000", this means no link.
    pub song_id: u64,
    // Bubble field
    /// This is length of Bubble field.
    /// ```txt
    /// 2^length
    /// ```
    pub length: u8,
    /// This is width of Bubble field.
    /// ```txt
    /// 2^width
    /// ```
    pub width: u8,
    /// This is height of Bubble field.
    /// ```txt
    /// 2^height
    /// ```
    pub height: u8,
    // Format
    /// Number of Bubble in Floaout
    pub bubbles: u16,
    /// This is number of block.
    pub blocks: u64,
    /// Sampling Rate
    pub sampling_rate: u32,
    /// Bits Per Sample
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

/// This structure contains data about Bubble in Floaout.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleInFloaout {
    /// Bubble Name Size
    pub name_size: u8,
    /// Name of Bubble
    pub name: String,
    // Color
    /// Red (0~255)
    pub red: u8,
    /// Green (0~255)
    pub green: u8,
    /// Blue (0~255)
    pub blue: u8
}

/// This structure covers all 'BubbleInFloaout' in Floaout.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubblesInFloaout(Vec<BubbleInFloaout>);

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