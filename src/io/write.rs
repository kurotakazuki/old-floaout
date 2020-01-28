use crate::format::bub::Bubble;
use crate::format::wav::Wav;
use std::io::{BufWriter, Result, Write};

pub trait WriteBytes<T>: Write {
    fn write_be_bytes(&mut self, _: T) -> Result<()>;
    fn write_le_bytes(&mut self, _: T) -> Result<()>;
}

// Maybe macro is better to write.

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

impl<W: Write + ?Sized> WriteBytes<Vec<Vec<Vec<u8>>>> for W {
    #[inline]
    fn write_be_bytes(&mut self, n: Vec<Vec<Vec<u8>>>) -> Result<()> {
        self.write_all(&n.concat().concat()[..])
    }

    #[inline]
    fn write_le_bytes(&mut self, n: Vec<Vec<Vec<u8>>>) -> Result<()> {
        n.concat().concat().reverse();
        self.write_all(&n.concat().concat()[..])
    }
}

pub trait WriteExt<T>: Write {
    fn write_details(&mut self, _: T) -> Result<()>;
}

impl<W: Write> WriteExt<Bubble> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, bub: Bubble) -> Result<()> {
        // Bubble
        self.write_be_bytes("oao")?;
        self.write_le_bytes(bub.version)?;
        // Bubble field
        self.write_le_bytes(bub.length)?;
        self.write_le_bytes(bub.width)?;
        self.write_le_bytes(bub.height)?;
        // Color
        self.write_le_bytes(bub.red)?;
        self.write_le_bytes(bub.green)?;
        self.write_le_bytes(bub.blue)?;
        // Format
        self.write_le_bytes(bub.blocks)?;
        self.write_le_bytes(bub.sampling_rate)?;
        self.write_le_bytes(bub.bits_per_sample)?;
        self.write_le_bytes(bub.name_size)?;
        self.write_be_bytes(bub.name)?;
        self.write_le_bytes(bub.overall)?;

        Ok(())
    }
}

impl<W: Write> WriteExt<Wav> for BufWriter<W> {
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