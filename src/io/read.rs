use std::io::Seek;
use crate::format::blow::{Blower, BubbleInBlower, BubblesInBlower};
use crate::format::bub::Bubble;
use crate::format::oao::{BubbleInFloaout, BubblesInFloaout, Floaout};
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

impl<R: Read + ?Sized> ReadBytes<(u64, u64)> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<(u64, u64)> {
        let mut bytes1 = [0; 8];
        let mut bytes2 = [0; 8];
        self.read_exact(&mut bytes1)?;
        self.read_exact(&mut bytes2)?;
        Ok((u64::from_be_bytes(bytes1), u64::from_be_bytes(bytes2)))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<(u64, u64)> {
        let mut bytes1 = [0; 8];
        let mut bytes2 = [0; 8];
        self.read_exact(&mut bytes1)?;
        self.read_exact(&mut bytes2)?;
        Ok((u64::from_le_bytes(bytes1), u64::from_le_bytes(bytes2)))
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


#[inline]
fn read_assert_eq<'a, R: Read + ?Sized>(this: &mut R, s: &'a str) -> Result<()> {
    let s2: String = this.read_be_bytes_for(s.len())?;
    assert_eq!(s, s2);

    Ok(())
}

pub trait ReadExt<T>: Read {
    fn read_details(&mut self) -> Result<T>;
}

impl<R: Read + Seek> ReadExt<Blower> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Blower> {
        // Initialized
        let mut blow = Blower::default();
        // Blower
        read_assert_eq(self, "blow")?;
        blow.version = self.read_le_bytes()?;
        // Bubble field
        blow.length = self.read_le_bytes()?;
        blow.width = self.read_le_bytes()?;
        blow.height = self.read_le_bytes()?;
        // Format
        blow.bubbles = self.read_le_bytes()?;
        blow.blocks = self.read_le_bytes()?;
        blow.sampling_rate = self.read_le_bytes()?;
        blow.bits_per_sample = self.read_le_bytes()?;

        Ok(blow)
    }
}

impl<R: Read + Seek> ReadExt<Bubble> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Bubble> {
        // Initialized
        let mut bub = Bubble::default();
        // Bubble
        read_assert_eq(self, "bub")?;
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

impl<R: Read + Seek> ReadExt<Floaout> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Floaout> {
        // Initialized
        let mut oao = Floaout::default();
        // Floaout
        read_assert_eq(self, "oao")?;
        oao.version = self.read_le_bytes()?;
        oao.song_id = self.read_le_bytes()?;
        // Bubble field
        oao.length = self.read_le_bytes()?;
        oao.width = self.read_le_bytes()?;
        oao.height = self.read_le_bytes()?;
        // Format
        oao.bubbles = self.read_le_bytes()?;
        oao.blocks = self.read_le_bytes()?;
        oao.sampling_rate = self.read_le_bytes()?;
        oao.bits_per_sample = self.read_le_bytes()?;

        Ok(oao)
    }
}

impl<R: Read + Seek> ReadExt<Wav> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Wav> {
        // Initialized
        let mut wav = Wav::default();
        // Repeat when there is a chunk.
        loop {
            let chunk_name: String = self.read_be_bytes_for(4)?;
            // Allocate by chunk name.
            match &*chunk_name {
                // RIFF
                "RIFF" => {
                    wav.riff_size = self.read_le_bytes()?;
                    read_assert_eq(self, "WAVE")?;
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

pub trait ReadExtFor<T>: Read {
    fn read_details_for(&mut self, times: usize) -> Result<T>;
}

impl<R: Read + Seek> ReadExtFor<BubblesInBlower> for BufReader<R> {
    #[inline]
    fn read_details_for(&mut self, times: usize) -> Result<BubblesInBlower> {
        // Into Vec
        let mut vec_of_bub_in_blow: Vec<BubbleInBlower> = Vec::new();
        for _ in 0..times {
            // Name of Bubble
            let name_size: u8 = self.read_le_bytes()?;
            let name = self.read_le_bytes_for(name_size as usize)?;
            // Times
            let times: u32 = self.read_le_bytes()?;
            // Ranges
            let mut ranges: Vec<(u64, u64)> = Vec::new();
            for _ in 0..times {
                ranges.push(
                    self.read_le_bytes()?
                );
            }
            vec_of_bub_in_blow.push(
                BubbleInBlower {
                    name_size,
                    name,
                    times,
                    ranges
                }
            );
        }

        Ok(BubblesInBlower::from(vec_of_bub_in_blow))
    }
}

impl<R: Read + Seek> ReadExtFor<BubblesInFloaout> for BufReader<R> {
    #[inline]
    fn read_details_for(&mut self, times: usize) -> Result<BubblesInFloaout> {
        // Into Vec
        let mut vec_of_bub_in_oao: Vec<BubbleInFloaout> = Vec::new();
        for _ in 0..times {
            let name_size: u8 = self.read_le_bytes()?;
            vec_of_bub_in_oao.push(
                BubbleInFloaout {
                    name_size,
                    name: self.read_be_bytes_for(name_size as usize)?,
                    red: self.read_le_bytes()?,
                    green: self.read_le_bytes()?,
                    blue: self.read_le_bytes()?,
                }
            );
        }

        Ok(BubblesInFloaout::from(vec_of_bub_in_oao))
    }
}