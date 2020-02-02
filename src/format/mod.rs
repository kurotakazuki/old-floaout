//! Format definitions
//! 
//! This module contains structures related to `Blower`, `Bubble`, `Floaout` and `Wav`.

pub mod blow;
pub mod bub;
pub mod oao;
pub mod wav;

/// This enum is type of sample format.
/// It's related to Bits Per Sample.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Sample {
    /// This type will match, if Bits Per Sample is 32.
    Float32(f32),
    /// This type will match, if Bits Per Sample is 64.
    Float64(f64)
}

impl From<f32> for Float32 {
    fn from(sample: f32) -> Self {
        Float32(sample)
    }
}

impl From<f64> for Float64 {
    fn from(sample: f64) -> Self {
        Float64(sample)
    }
}