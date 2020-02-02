//! Write formats

use crate::format::{BubbleField, BubbleFieldSize, Color};
use crate::format::blow::{Blower, BubbleInBlower, BubblesInBlower};
use crate::format::bub::Bubble;
use crate::format::oao::{BubbleInFloaout, BubblesInFloaout, Floaout};
use crate::format::wav::Wav;
use std::convert::TryInto;
use std::io::{BufWriter, Result, Write};

/// This trait writes bytes for inferring from variable.
pub trait WriteBytes<T>: Write {
    /// This method writes bytes in big-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::write::WriteBytes;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // write u32 in big-endian byte order
    ///     let u_32: u32 = 1;
    ///     f.write_be_bytes(u_32)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_be_bytes(&mut self, _: T) -> Result<()>;
    /// This method writes bytes in little-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::write::WriteBytes;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // write u32 in little-endian byte order
    ///     let u_32: u32 = 1;
    ///     f.write_le_bytes(u_32)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_le_bytes(&mut self, _: T) -> Result<()>;
}

// Maybe macro is better to write.

impl<W: Write + ?Sized> WriteBytes<Color> for W {
    #[inline]
    fn write_be_bytes(&mut self, color: Color) -> Result<()> {
        let (red, green, blue) = color.into();

        self.write_all(&red.to_be_bytes())?;
        self.write_all(&green.to_be_bytes())?;
        self.write_all(&blue.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, color: Color) -> Result<()> {
        let (red, green, blue) = color.into();

        self.write_all(&red.to_le_bytes())?;
        self.write_all(&green.to_le_bytes())?;
        self.write_all(&blue.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<BubbleFieldSize> for W {
    #[inline]
    fn write_be_bytes(&mut self, bub_field_size: BubbleFieldSize) -> Result<()> {
        let (length, width, height) = bub_field_size.into();

        self.write_all(&length.to_be_bytes())?;
        self.write_all(&width.to_be_bytes())?;
        self.write_all(&height.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, bub_field_size: BubbleFieldSize) -> Result<()> {
        let (length, width, height) = bub_field_size.into();

        self.write_all(&length.to_le_bytes())?;
        self.write_all(&width.to_le_bytes())?;
        self.write_all(&height.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<f32> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: f32) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: f32) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<f64> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: f64) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: f64) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<&str> for W {
    #[inline]
    fn write_be_bytes(&mut self, s: &str) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, s: &str) -> Result<()> {
        self.write_all(s.chars().rev().collect::<String>().as_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<String> for W {
    #[inline]
    fn write_be_bytes(&mut self, s: String) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, s: String) -> Result<()> {
        self.write_all(s.chars().rev().collect::<String>().as_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<u8> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: u8) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: u8) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<u16> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: u16) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: u16) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<u32> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: u32) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: u32) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<u64> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: u64) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, n: u64) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: Write + ?Sized> WriteBytes<(u64, u64)> for W {
    #[inline]
    fn write_be_bytes(&mut self, (n1, n2): (u64, u64)) -> Result<()> {
        self.write_all(&n1.to_be_bytes())?;
        self.write_all(&n2.to_be_bytes())
    }

    #[inline]
    fn write_le_bytes(&mut self, (n1, n2): (u64, u64)) -> Result<()> {
        self.write_all(&n1.to_le_bytes())?;
        self.write_all(&n2.to_le_bytes())
    }
}

#[inline]
fn write_bubble_field<W: Write + ?Sized>(this: &mut W, bub_field: BubbleField, bub_field_size: BubbleFieldSize) -> Result<()> {
    let (length, width, height) = bub_field_size.try_into().expect("failed BubbbleFieldSize into tuple");
    let n: Vec<Vec<Vec<u8>>> = bub_field.into();
    for height in 0..height {
        for width in 0..width {
            for length in 0..length {
                this.write_le_bytes(n[length][width][height])?;
            }
        }
    }

    Ok(())
}

/// This trait writes format.
pub trait WriteFmt<T>: Write {
    /// This method writes details of format.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::format::bub::Bubble;
    /// use floaout::io::write::WriteFmt;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut writer = io::BufWriter::new(File::create("foo.bub")?);
    /// 
    ///     // write Bubble details
    ///     let bub: Bubble = Default::default();
    ///     writer.write_details(bub)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_details(&mut self, _: T) -> Result<()>;
}

impl<W: Write> WriteFmt<Blower> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, blow: Blower) -> Result<()> {
        // Blower
        self.write_be_bytes("blow")?;
        self.write_le_bytes(blow.version)?;
        // Bubble field size
        self.write_le_bytes(blow.bub_field_size)?;
        // Format
        self.write_le_bytes(blow.bubbles)?;
        self.write_le_bytes(blow.blocks)?;
        self.write_le_bytes(blow.sampling_rate)?;
        self.write_le_bytes(blow.bits_per_sample)?;

        Ok(())
    }
}

impl<W: Write> WriteFmt<Bubble> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, bub: Bubble) -> Result<()> {
        // Bubble
        self.write_be_bytes("bub")?;
        self.write_le_bytes(bub.version)?;
        // Bubble field size
        self.write_le_bytes(bub.bub_field_size)?;
        // Color
        self.write_le_bytes(bub.color)?;
        // Format
        self.write_le_bytes(bub.blocks)?;
        self.write_le_bytes(bub.sampling_rate)?;
        self.write_le_bytes(bub.bits_per_sample)?;
        self.write_le_bytes(bub.name_size)?;
        self.write_be_bytes(bub.name)?;
        write_bubble_field(self, bub.overall, bub.bub_field_size)?;

        Ok(())
    }
}

impl<W: Write> WriteFmt<Floaout> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, oao: Floaout) -> Result<()> {
        // Floaout
        self.write_be_bytes("oao")?;
        self.write_le_bytes(oao.version)?;
        self.write_le_bytes(oao.song_id)?;
        // Bubble field size
        self.write_le_bytes(oao.bub_field_size)?;
        // Format
        self.write_le_bytes(oao.bubbles)?;
        self.write_le_bytes(oao.blocks)?;
        self.write_le_bytes(oao.sampling_rate)?;
        self.write_le_bytes(oao.bits_per_sample)?;

        Ok(())
    }
}

impl<W: Write> WriteFmt<Wav> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, wav: Wav) -> Result<()> {
        // Riff Chunk
        self.write_be_bytes("RIFF")?;
        self.write_le_bytes(wav.data_size + 4 + 8 + wav.format_size + 8)?;
        self.write_be_bytes("WAVE")?;
        // Format Chunk
        self.write_be_bytes("fmt ")?;
        self.write_le_bytes(wav.format_size)?;
        self.write_le_bytes(wav.format_tag)?;
        self.write_le_bytes(wav.channels)?;
        self.write_le_bytes(wav.sampling_rate)?;
        self.write_le_bytes(wav.data_rate)?;
        self.write_le_bytes(wav.data_block_size)?;
        self.write_le_bytes(wav.bits_per_sample)?;
        // Data Chunk
        self.write_be_bytes("data")?;
        self.write_le_bytes(wav.data_size)?;

        Ok(())
    }

}

/// This trait writes Bubbles in format.
pub trait WriteBubsIn<T>: Write {
    /// This method writes details of Bubbles in format.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::format::oao::BubblesInFloaout;
    /// use floaout::io::write::WriteBubsIn;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut writer = io::BufWriter::new(File::create("foo.oao")?);
    /// 
    ///     // write BubbleInFloaout details
    ///     let bubs_in_oao: BubblesInFloaout = Default::default();
    ///     writer.write_bubs_details(bubs_in_oao)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_bubs_details(&mut self, _: T) -> Result<()>;
}

impl<W: Write> WriteBubsIn<BubblesInBlower> for BufWriter<W> {
    #[inline]
    fn write_bubs_details(&mut self, bubs_in_blow: BubblesInBlower) -> Result<()> {
        // Into Vec
        let vec_of_bub_in_blow: Vec<BubbleInBlower> = bubs_in_blow.into();
        for bub_in_blow in vec_of_bub_in_blow {
            // Name of Bubble
            self.write_le_bytes(bub_in_blow.name_size)?;
            self.write_be_bytes(bub_in_blow.name)?;
            // Times
            self.write_le_bytes(bub_in_blow.times)?;
            // Ranges
            for range in bub_in_blow.ranges {
                self.write_le_bytes(range)?;
            }
        }

        Ok(())
    }
}

impl<W: Write> WriteBubsIn<BubblesInFloaout> for BufWriter<W> {
    #[inline]
    fn write_bubs_details(&mut self, bubs_in_oao: BubblesInFloaout) -> Result<()> {
        // Into Vec
        let vec_of_bub_in_oao: Vec<BubbleInFloaout> = bubs_in_oao.into();
        for bub_in_oao in vec_of_bub_in_oao {
            // Name of Bubble
            self.write_le_bytes(bub_in_oao.name_size)?;
            self.write_be_bytes(bub_in_oao.name)?;
            // Color
            self.write_le_bytes(bub_in_oao.color)?;
        }

        Ok(())
    }
}