use std::io::Seek;
use crate::format::bub::Bubble;
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

pub trait ReadBytesFor<T>: Read {
    fn read_be_bytes_for(&mut self, size: usize) -> Result<T>;
    fn read_le_bytes_for(&mut self, size: usize) -> Result<T>;
}

impl<R: Read + ?Sized> ReadBytesFor<Vec<u8>> for R {
    #[inline]
    fn read_be_bytes_for(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut bytes = [0; 1];
        for _ in 0..size {
            self.read_exact(&mut bytes)?;
            buf.push(bytes[0]);
        }

        Ok(buf)
    }

    #[inline]
    fn read_le_bytes_for(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut bytes = [0; 1];
        for _ in 0..size {
            self.read_exact(&mut bytes)?;
            buf.push(bytes[0]);
        }
        buf.reverse();

        Ok(buf)
    }
}

impl<R: Read + ?Sized> ReadBytesFor<String> for R {
    #[inline]
    fn read_be_bytes_for(&mut self, size: usize) -> Result<String> {
        let buf: Vec<u8> = self.read_be_bytes_for(size)?;

        Ok(std::string::String::from_utf8(buf).unwrap())
    }

    #[inline]
    fn read_le_bytes_for(&mut self, size: usize) -> Result<String> {
        let buf: Vec<u8> = self.read_le_bytes_for(size)?;

        Ok(std::string::String::from_utf8(buf).unwrap())
    }
}

#[inline]
fn read_bubble_field<R: Read + ?Sized>(this: &mut R, length: u8, width: u8, height: u8) -> Result<Vec<Vec<Vec<u8>>>> {
    let length = 1 << length;
    let width = 1 << width;
    let height = 1 << height;
    let mut bubble_field: Vec<Vec<Vec<u8>>> =  vec![vec![vec![0; height]; width]; length];
    for height in 0..height {
        for width in 0..width {
            for length in 0..length {
                bubble_field[length][width][height] = this.read_le_bytes()?;
            }
        }
    }

    Ok(bubble_field)
}

pub trait ReadExt<T>: Read {
    fn read_details(&mut self) -> Result<T>;
}

impl<R: Read + Seek> ReadExt<Bubble> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Bubble> {
        // Initialized
        let mut bub = Bubble::default();
        // Bubble
        self.seek_relative(3)?;
        bub.version = self.read_le_bytes()?;
        // Bubble field
        bub.length = self.read_le_bytes()?;
        bub.width = self.read_le_bytes()?;
        bub.height = self.read_le_bytes()?;
        // Color
        bub.red = self.read_le_bytes()?;
        bub.green = self.read_le_bytes()?;
        bub.blue = self.read_le_bytes()?;
        // Format
        bub.blocks = self.read_le_bytes()?;
        bub.sampling_rate = self.read_le_bytes()?;
        bub.bits_per_sample = self.read_le_bytes()?;
        bub.name_size = self.read_le_bytes()?;
        bub.name = self.read_be_bytes_for(bub.name_size as usize)?;
        bub.overall = read_bubble_field(self, bub.length, bub.width, bub.height)?;

        Ok(bub)
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
        // Initialized
        let mut wav = Wav::default();
        // Repeat when there is a chunk.
        loop {
            let chunk_name: String = self.read_be_bytes_for(4)?;
            // Allocate by chunk name.
            match &chunk_name[..] {
                // RIFF
                "RIFF" => {
                    wav.riff_size = self.read_le_bytes()?;
                    self.seek_relative(4)?;
                },
                // Format
                "fmt " => {
                    wav.format_size = self.read_le_bytes()?;
                    wav.format_tag = self.read_le_bytes()?;
                    wav.channels = self.read_le_bytes()?;
                    wav.sampling_rate = self.read_le_bytes()?;
                    wav.data_rate = self.read_le_bytes()?;
                    wav.data_block_size = self.read_le_bytes()?;
                    wav.bits_per_sample = self.read_le_bytes()?;
                },
                // Data
                "data" => {
                    wav.data_size = self.read_le_bytes()?;
                    break
                },
                // Other
                _ => {
                    let chunk_size: u32 = self.read_le_bytes()?;
                    // Add 8 and chunk_size bytes to other_size.
                    wav.other_size += 8 + chunk_size;
                    self.seek_relative(chunk_size as i64)?;
                },
            }
        }

        Ok(wav)
    }
}