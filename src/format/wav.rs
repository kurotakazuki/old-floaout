use std::fmt;

/// Details of the Wav file.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Wav {
    // Riff Chunk
    pub riff_size: u32,
    // Format Chunk
    pub format_size: u32,
    pub format_tag: u16,
    pub channels: u16,
    pub sampling_rate: u32,
    pub data_rate: u32,
    pub data_block_size: u16,
    pub bits_per_sample: u16,
    // Data Chunk
    pub data_size: u32,
    // Other Chunk
    pub other_size: u32
}

impl fmt::Display for Wav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "\n File Size( - 8 ): {} Bytes\nFormat Chunk Size: {} Bytes\n      Format Tag: {}\n         Channels: {} Channels\n    Sampling Rate: {} Hz\n        Data Rate: {} kbps\n  Data Block Size: {} Bytes\n  Bits Per Sample: {} Bits\n   Wave Data Size: {} Bytes\n Other Chunk Size: {} Bytes\n",
            self.riff_size,
            self.format_size,
            self.format_tag,
            self.channels,
            self.sampling_rate,
            self.data_rate,
            self.data_block_size,
            self.bits_per_sample,
            self.data_size,
            self.other_size
        )
    }
}