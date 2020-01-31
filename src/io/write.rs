use crate::format::blow::Blower;
use crate::format::bub::Bubble;
use crate::format::oao::Floaout;
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

impl<W: Write> WriteExt<Blower> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, blow: Blower) -> Result<()> {
        // Blower
        self.write_be_bytes("blow")?;
        self.write_le_bytes(blow.version)?;
        // Bubble field
        self.write_le_bytes(blow.length)?;
        self.write_le_bytes(blow.width)?;
        self.write_le_bytes(blow.height)?;
        // Format
        self.write_le_bytes(blow.bubbles)?;
        self.write_le_bytes(blow.blocks)?;
        self.write_le_bytes(blow.sampling_rate)?;
        self.write_le_bytes(blow.bits_per_sample)?;

        Ok(())
    }
}

impl<W: Write> WriteExt<Bubble> for BufWriter<W> {
    /// # Examples
    /// 
    ///```
    /// let bub = Bubble {
    ///     version: 0,
    ///     length: 1,
    ///     width: 1,
    ///     height: 1,
    ///     red: 255,
    ///     green: 255,
    ///     blue: 255,
    ///     blocks: 8,
    ///     sampling_rate: 44100,
    ///     bits_per_sample: 32,
    ///     name_size: 5,
    ///     name: "hello".to_string(),
    ///     overall: vec![vec![vec![0, 1], vec![2, 3]], vec![vec![4, 5], vec![6, 7]], vec![vec![8, 9], vec![10, 11]], vec![vec![12, 13], vec![14, 15]]]
    /// };
    /// 
    /// let  bub_file = "new.bub";
    /// let mut writer = BufWriter::new(File::create(bub_file)?);
    /// 
    /// writer.write_details(bub)?;
    ///```
    #[inline]
    fn write_details(&mut self, bub: Bubble) -> Result<()> {
        // Bubble
        self.write_be_bytes("bub")?;
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

impl<W: Write> WriteExt<Floaout> for BufWriter<W> {
    #[inline]
    fn write_details(&mut self, oao: Floaout) -> Result<()> {
        // Floaout
        self.write_be_bytes("oao")?;
        self.write_le_bytes(oao.version)?;
        self.write_le_bytes(oao.song_id)?;
        // Bubble field
        self.write_le_bytes(oao.length)?;
        self.write_le_bytes(oao.width)?;
        self.write_le_bytes(oao.height)?;
        // Format
        self.write_le_bytes(oao.bubbles)?;
        self.write_le_bytes(oao.blocks)?;
        self.write_le_bytes(oao.sampling_rate)?;
        self.write_le_bytes(oao.bits_per_sample)?;

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