//! Format definitions
//! 
//! This module contains structures related to `Blower`, `Bubble`, `Floaout` and `Wav`.

pub mod blow;
pub mod bub;
pub mod oao;
pub mod wav;

use std::convert::TryInto;

/// This structure is each value of Bubble field.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleFieldSize {
    length: u8,
    width: u8,
    height: u8
}

impl From<(usize, usize, usize)> for BubbleFieldSize {
    fn from((length, width, height): (usize, usize, usize)) -> Self {
        Self {
            length: (length as f32).log2() as u8,
            width: (width as f32).log2() as u8,
            height: (height as f32).log2() as u8
        }
    }
}

impl TryInto<(usize, usize, usize)> for BubbleFieldSize {
    type Error = &'static str;

    fn try_into(self) -> Result<(usize, usize, usize), Self::Error> {
        // convert
        let (length, a) = 2usize.overflowing_pow(self.length as u32);
        let (width, b) = 2usize.overflowing_pow(self.width as u32);
        let (height, c) = 2usize.overflowing_pow(self.height as u32);
        // check whether overflowing
        if a || b || c {
            Err("One of the following: length, width and height is overflowing.")
        } else {
            Ok((length, width, height))
        }
    }
}

/// This enum is type of sample format.
/// It's related to Bits Per Sample.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Sample {
    /// This type will match, if Bits Per Sample is 32.
    Float32(f32),
    /// This type will match, if Bits Per Sample is 64.
    Float64(f64)
}

impl From<f32> for Sample {
    fn from(sample: f32) -> Self {
        Sample::Float32(sample)
    }
}

impl From<f64> for Sample {
    fn from(sample: f64) -> Self {
        Sample::Float64(sample)
    }
}