use std::io::Seek;
use crate::format::wav::Wav;
use std::io::{BufReader, Read, Result};

pub trait ReadBytes<T>: Read {
    fn read_be_bytes(&mut self) -> Result<T>;
    fn read_le_bytes(&mut self) -> Result<T>;
}

// Maybe macro is better to write.

impl<R: Read + ?Sized> ReadBytes<f32> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<f32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<f32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }
}

impl<R: Read + ?Sized> ReadBytes<f64> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }
}

impl<R: Read + ?Sized> ReadBytes<u8> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_le_bytes(bytes))
    }
}

impl<R: Read + ?Sized> ReadBytes<u16> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }
}

impl<R: Read + ?Sized> ReadBytes<u32> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }
}


impl<R: Read + ?Sized> ReadBytes<u64> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<u64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<u64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

pub trait ReadExt<T>: Read {
    fn read_details(&mut self) -> Result<T>;

    #[inline]
    fn read_for(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut bytes = [0; 1];
        for _ in 0..size {
            self.read_exact(&mut bytes)?;
            buf.push(bytes[0]);
        }

        Ok(buf)
    }

    #[inline]
    fn read_string_for(&mut self, size: usize) -> Result<String> {
        let buf = self.read_for(size)?;

        Ok(std::string::String::from_utf8(buf).unwrap())
    }
}

impl<R: Read + Seek> ReadExt<Wav> for BufReader<R> {
    /// # Examples
    ///
    /// ```
    /// use std::io::BufReader;
    /// use std::fs::File;
    /// use floaout::format::wav::Wav;
    /// use floaout::io::read::ReadExt;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let file = "sample.wav";
    ///     let mut reader = BufReader::new(File::open(file)?);
    /// 
    ///     let a: Wav = reader.read_details()?;
    /// 
    ///     println!("{}", a);
    /// 
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    fn read_details(&mut self) -> Result<Wav> {
        let mut details = Wav::default();
        // Repeat when there is a chunk.
        loop {
            let chunk_name = self.read_string_for(4)?;
            // Allocate by chunk name.
            match &chunk_name[..] {
                // RIFF
                "RIFF" => {
                    details.riff_size = self.read_le_bytes()?;
                    self.seek_relative(4)?;
                },
                // Format
                "fmt " => {
                    details.format_size = self.read_le_bytes()?;
                    details.format_code = self.read_le_bytes()?;
                    details.channels = self.read_le_bytes()?;
                    details.sampling_rate = self.read_le_bytes()?;
                    details.data_rate = self.read_le_bytes()?;
                    details.data_block_size = self.read_le_bytes()?;
                    details.bits_per_sample = self.read_le_bytes()?;
                },
                // Data
                "data" => {
                    details.data_size = self.read_le_bytes()?;
                    break
                },
                // Other
                _ => {
                    let chunk_size: u32 = self.read_le_bytes()?;
                    // Add 8 and chunk_size bytes to other_size.
                    details.other_size += 8 + chunk_size;
                    self.seek_relative(chunk_size as i64)?;
                },
            }
        }

        Ok(details)
    }
}