//! Write formats

use crate::format::{BubbleField, BubbleFieldSize, Color, Sample};
use crate::format::bub::{Bubble, BubbleBlock, BubbleBlocks};
use crate::format::oao::{BubblesInFloaout, Floaout, FloaoutBlock, FloaoutBlocks};
use crate::format::wav::{Wav, WavBlock, WavBlocks};
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

impl<W: Write + ?Sized> WriteBytes<Sample> for W {
    #[inline]
    fn write_be_bytes(&mut self, sample: Sample) -> Result<()> {
        match sample {
            Sample::Float32(n) => self.write_be_bytes(n),
            Sample::Float64(n) => self.write_be_bytes(n)
        }
    }

    #[inline]
    fn write_le_bytes(&mut self, sample: Sample) -> Result<()> {
        match sample {
            Sample::Float32(n) => self.write_le_bytes(n),
            Sample::Float64(n) => self.write_le_bytes(n)
        }
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

/// This trait writes block.
pub trait WriteBlock<T, B>: Write {
    /// This method writes format block.
    fn write_block(&mut self, _: T, _: B) -> Result<()>;
}

impl<W: Write + ?Sized> WriteBlock<&Bubble, &BubbleBlock> for W {
    #[inline]
    fn write_block(&mut self, bub: &Bubble, bub_block: &BubbleBlock) -> Result<()> {
        self.write_block(&Wav::default(), bub_block.wav_block)?;
        write_bubble_field(self, bub_block.bub_field.clone(), bub.bub_field_size)
    }
}

impl<W: Write + ?Sized> WriteBlock<&Floaout, &FloaoutBlock> for W {
    #[inline]
    fn write_block(&mut self, oao: &Floaout, oao_block: &FloaoutBlock) -> Result<()> {
        let bub = Bubble::from(*oao);
        for bub_block in &oao_block.0 {
            self.write_block(&bub, &bub_block)?;
        }

        Ok(())
    }
}

impl<W: Write + ?Sized> WriteBlock<&Wav, WavBlock> for W {
    #[inline]
    fn write_block(&mut self, _wav: &Wav, wav_block: WavBlock) -> Result<()> {
        let sample: Sample = wav_block.into();
        self.write_le_bytes(sample)
    }
}

/// This trait writes format.
pub trait WriteFmt<T, B>: Write {
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
    ///     writer.write_details(&bub)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_details(&mut self, _: &T) -> Result<()>;
    /// This method writes format blocks.
    fn write_blocks(&mut self, _: &T, _: B) -> Result<()>;
}

impl<W: Write> WriteFmt<Bubble, BubbleBlocks> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, bub: &Bubble) -> Result<()> {
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
        self.write_be_bytes(bub.name.clone())?;
        write_bubble_field(self, bub.overall.clone(), bub.bub_field_size)?;

        Ok(())
    }

    #[inline]
    fn write_blocks(&mut self, bub: &Bubble, bub_blocks: BubbleBlocks) -> Result<()> {
        for bub_block in &*bub_blocks.0 {
            self.write_block(bub, bub_block)?;
        }

        Ok(())
    }
}

impl<W: Write> WriteFmt<Floaout, FloaoutBlocks> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, oao: &Floaout) -> Result<()> {
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

    #[inline]
    fn write_blocks(&mut self, oao: &Floaout, oao_blocks: FloaoutBlocks) -> Result<()> {
        for oao_block in &*oao_blocks.0 {
            self.write_block(oao, oao_block)?;
        }

        Ok(())
    }
}

impl<W: Write> WriteFmt<Wav, WavBlocks> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, wav: &Wav) -> Result<()> {
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

    #[inline]
    fn write_blocks(&mut self, wav: &Wav, wav_blocks: WavBlocks) -> Result<()> {
        for wav_block in &*wav_blocks.0 {
            self.write_block(wav, *wav_block)?;
        }

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
    ///     writer.write_bubs_details(&bubs_in_oao)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn write_bubs_details(&mut self, _: &T) -> Result<()>;
}

impl<W: Write> WriteBubsIn<BubblesInFloaout> for BufWriter<W> {
    #[inline]
    fn write_bubs_details(&mut self, bubs_in_oao: &BubblesInFloaout) -> Result<()> {
        for bub_in_oao in &*bubs_in_oao.0 {
            // Name of Bubble
            self.write_le_bytes(bub_in_oao.name_size)?;
            self.write_be_bytes(bub_in_oao.name.clone())?;
            // Color
            self.write_le_bytes(bub_in_oao.color)?;
        }

        Ok(())
    }
}