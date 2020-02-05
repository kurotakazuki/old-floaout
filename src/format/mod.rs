//! Format definitions
//! 
//! This module contains structures related to `Blower`, `Bubble`, `Floaout` and `Wav`.

pub mod bub;
pub mod oao;
pub mod wav;

use std::convert::TryInto;

/// This structure is value of Bubble field.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleField(Vec<Vec<Vec<u8>>>);

impl From<Vec<Vec<Vec<u8>>>> for BubbleField {
    fn from(bub_field: Vec<Vec<Vec<u8>>>) -> Self {
        BubbleField(bub_field)
    }
}

impl Into<Vec<Vec<Vec<u8>>>> for BubbleField {
    fn into(self) -> Vec<Vec<Vec<u8>>> {
        self.0
    }
}

/// This structure is each size of Bubble field.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct BubbleFieldSize {
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
}

impl From<(u8, u8, u8)> for BubbleFieldSize {
    fn from((length, width, height): (u8, u8, u8)) -> Self {
        Self {
            length,
            width,
            height
        }
    }
}

impl Into<(u8, u8, u8)> for BubbleFieldSize {
    fn into(self) -> (u8, u8, u8) {
        (self.length, self.width, self.height)
    }
}

// This will change inside of number.
impl From<(usize, usize, usize)> for BubbleFieldSize {
    fn from((length, width, height): (usize, usize, usize)) -> Self {
        Self {
            length: (length as f32).log2() as u8,
            width: (width as f32).log2() as u8,
            height: (height as f32).log2() as u8
        }
    }
}

// This will change inside of number.
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

/// This structure is color of Bubble.
#[derive(Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Color{
    /// Red (0~255)
    pub red: u8,
    /// Green (0~255)
    pub green: u8,
    /// Blue (0~255)
    pub blue: u8
}

impl From<(u8, u8, u8)> for Color {
    fn from((red, blue, green): (u8, u8, u8)) -> Self {
        Self {
            red,
            blue,
            green
        }
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
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