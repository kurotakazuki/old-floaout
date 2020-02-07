//! Read formats

use std::io::Seek;
use crate::format::{BubbleField, BubbleFieldSize, Color, Sample};
use crate::format::bub::{Bubble, BubbleBlock, BubbleBlocks};
use crate::format::oao::{BubbleInFloaout, BubblesInFloaout, Floaout, FloaoutBlock, FloaoutBlocks};
use crate::format::wav::{Wav, WavBlock, WavBlocks};
use std::convert::TryInto;
use std::io::{BufReader, Read, Result};

/// This trait reads bytes for inferring from variable to be assigned.
pub trait ReadBytes<T>: Read {
    /// This method reads bytes in big-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::read::ReadBytes;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // read u32 in big-endian byte order
    ///     let u_32: u32 = f.read_be_bytes()?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn read_be_bytes(&mut self) -> Result<T>;
    /// This method reads bytes in little-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::read::ReadBytes;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // read u32 in little-endian byte order
    ///     let u_32: u32 = f.read_le_bytes()?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn read_le_bytes(&mut self) -> Result<T>;
}

// Maybe macro is better to write.

impl<R: Read + ?Sized> ReadBytes<BubbleFieldSize> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<BubbleFieldSize> {
        let mut length = [0; 1];
        self.read_exact(&mut length)?;
        let mut width = [0; 1];
        self.read_exact(&mut width)?;
        let mut height = [0; 1];
        self.read_exact(&mut height)?;
        Ok(
            BubbleFieldSize {
                length: u8::from_be_bytes(length),
                width: u8::from_be_bytes(width),
                height: u8::from_be_bytes(height)
            }
        )
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<BubbleFieldSize> {
        let mut length = [0; 1];
        self.read_exact(&mut length)?;
        let mut width = [0; 1];
        self.read_exact(&mut width)?;
        let mut height = [0; 1];
        self.read_exact(&mut height)?;
        Ok(
            BubbleFieldSize {
                length: u8::from_le_bytes(length),
                width: u8::from_le_bytes(width),
                height: u8::from_le_bytes(height)
            }
        )
    }
}

impl<R: Read + ?Sized> ReadBytes<Color> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> Result<Color> {
        let mut red = [0; 1];
        self.read_exact(&mut red)?;
        let mut green = [0; 1];
        self.read_exact(&mut green)?;
        let mut blue = [0; 1];
        self.read_exact(&mut blue)?;
        Ok(
            Color {
                red: u8::from_be_bytes(red),
                green: u8::from_be_bytes(green),
                blue: u8::from_be_bytes(blue)
            }
        )
    }

    #[inline]
    fn read_le_bytes(&mut self) -> Result<Color> {
        let mut red = [0; 1];
        self.read_exact(&mut red)?;
        let mut green = [0; 1];
        self.read_exact(&mut green)?;
        let mut blue = [0; 1];
        self.read_exact(&mut blue)?;
        Ok(
            Color {
                red: u8::from_le_bytes(red),
                green: u8::from_le_bytes(green),
                blue: u8::from_le_bytes(blue)
            }
        )
    }
}

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

/// This trait reads bytes for size.
/// This is because, size is unknown in String.
pub trait ReadBytesFor<T>: Read {
    /// This method reads bytes in big-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::read::ReadBytesFor;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // read String for size of 3 in big-endian byte order
    ///     let string: String = f.read_be_bytes_for(3)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn read_be_bytes_for(&mut self, size: usize) -> Result<T>;
    /// This method reads bytes in little-endian byte order.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::io::read::ReadBytesFor;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut f = File::open("foo.txt")?;
    /// 
    ///     // read Vec<u8> for size of 1 in little-endian byte order
    ///     let vec_u8: Vec<u8> = f.read_le_bytes_for(1)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
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

impl<R: Read + ?Sized> ReadBytesFor<Sample> for R {
    #[inline]
    fn read_be_bytes_for(&mut self, size: usize) -> Result<Sample> {
        match size {
            4 => Ok(Sample::Float32(self.read_be_bytes()?)),
            8 => Ok(Sample::Float64(self.read_be_bytes()?)),
            // This should be change to Error type.
            _ => Ok(Sample::Float32(0.0))
        }
    }

    #[inline]
    fn read_le_bytes_for(&mut self, size: usize) -> Result<Sample> {
        match size {
            4 => Ok(Sample::Float32(self.read_le_bytes()?)),
            8 => Ok(Sample::Float64(self.read_le_bytes()?)),
            // This should be change to Error type.
            _ => Ok(Sample::Float32(0.0))
        }
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
fn read_bubble_field<R: Read + ?Sized>(this: &mut R, bub_field_size: BubbleFieldSize) -> Result<BubbleField> {
    let (length, width, height) = bub_field_size.try_into().expect("failed BubbbleFieldSize into tuple");
    let mut bubble_field: Vec<Vec<Vec<u8>>> =  vec![vec![vec![0; height]; width]; length];
    for height in 0..height {
        for width in 0..width {
            for length in 0..length {
                bubble_field[length][width][height] = this.read_le_bytes()?;
            }
        }
    }

    Ok(bubble_field.into())
}


#[inline]
fn read_assert_eq<'a, R: Read + ?Sized>(this: &mut R, s: &'a str) -> Result<()> {
    let s2: String = this.read_be_bytes_for(s.len())?;
    assert_eq!(s, s2);

    Ok(())
}

/// This trait reads block.
pub trait ReadBlock<T, B>: Read {
    /// This method reads format block.
    fn read_block(&mut self, _: T) -> Result<B>;
}

impl<R: Read + ?Sized> ReadBlock<&Bubble, BubbleBlock> for R {
    #[inline]
    fn read_block(&mut self, bub: &Bubble) -> Result<BubbleBlock> {
        let wav = Wav::from_bits_per_sample(bub.bits_per_sample);
        let wav_block = self.read_block(&wav)?;
        let bub_field = read_bubble_field(self, bub.bub_field_size)?;

        Ok(
            BubbleBlock {
                wav_block,
                bub_field
            }
        )
    }
}

impl<R: Read + ?Sized> ReadBlock<&Floaout, FloaoutBlock> for R {
    #[inline]
    fn read_block(&mut self, oao: &Floaout) -> Result<FloaoutBlock> {
        let mut bub_block_vec = Vec::new();
        let bub = Bubble::from_bub_field_size_and_bits_per_sample(oao.bub_field_size, oao.bits_per_sample);
        for _ in 0..oao.bubbles {
            let bub_block = self.read_block(&bub)?;
            bub_block_vec.push(bub_block);
        }

        Ok(bub_block_vec.into())
    }
}

impl<R: Read + ?Sized> ReadBlock<&Wav, WavBlock> for R {
    #[inline]
    fn read_block(&mut self, wav: &Wav) -> Result<WavBlock> {
        match wav.bits_per_sample {
            32 => Ok(WavBlock(self.read_le_bytes_for(4)?)),
            64 => Ok(WavBlock(self.read_le_bytes_for(8)?)),
            // This should be change to Error type.
            _ => Ok(WavBlock(0.0.into()))
        }
    }
}

/// This trait reads format.
pub trait ReadFmt<T, B>: Read {
    /// This method reads details of format.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::format::bub::Bubble;
    /// use floaout::io::read::ReadFmt;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut reader = io::BufReader::new(File::open("foo.bub")?);
    /// 
    ///     // read Bubble details
    ///     let bub: Bubble = reader.read_details()?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn read_details(&mut self) -> Result<T>;
    /// This method reads format blocks.
    fn read_blocks(&mut self, _: &T) -> Result<B>;
}

impl<R: Read + Seek> ReadFmt<Bubble, BubbleBlocks> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Bubble> {
        // Initialized
        let mut bub = Bubble::default();
        // Bubble
        read_assert_eq(self, "bub")?;
        bub.version = self.read_le_bytes()?;
        // Bubble field size
        bub.bub_field_size = self.read_le_bytes()?;
        // Color
        bub.color = self.read_le_bytes()?;
        // Format
        bub.blocks = self.read_le_bytes()?;
        bub.sampling_rate = self.read_le_bytes()?;
        bub.bits_per_sample = self.read_le_bytes()?;
        bub.name_size = self.read_le_bytes()?;
        bub.name = self.read_be_bytes_for(bub.name_size as usize)?;
        bub.overall = read_bubble_field(self, bub.bub_field_size)?;

        Ok(bub)
    }

    #[inline]
    fn read_blocks(&mut self, bub: &Bubble) -> Result<BubbleBlocks> {
        let mut bub_block_vec = Vec::<BubbleBlock>::with_capacity(bub.blocks as usize);
        for _ in 0..bub.blocks {
            let bub_block = self.read_block(bub)?;
            bub_block_vec.push(bub_block);
        }

        Ok(bub_block_vec.into_boxed_slice().into())
    }
}

impl<R: Read + Seek> ReadFmt<Floaout, FloaoutBlocks> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Floaout> {
        // Initialized
        let mut oao = Floaout::default();
        // Floaout
        read_assert_eq(self, "oao")?;
        oao.version = self.read_le_bytes()?;
        oao.song_id = self.read_le_bytes()?;
        // Bubble field size
        oao.bub_field_size = self.read_le_bytes()?;
        // Format
        oao.bubbles = self.read_le_bytes()?;
        oao.blocks = self.read_le_bytes()?;
        oao.sampling_rate = self.read_le_bytes()?;
        oao.bits_per_sample = self.read_le_bytes()?;

        Ok(oao)
    }

    #[inline]
    fn read_blocks(&mut self, oao: &Floaout) -> Result<FloaoutBlocks> {
        let mut oao_block_vec = Vec::<FloaoutBlock>::with_capacity(oao.blocks as usize);
        for _ in 0..oao.blocks {
            let oao_block = self.read_block(oao)?;
            oao_block_vec.push(oao_block);
        }

        Ok(oao_block_vec.into_boxed_slice().into())
    }
}

impl<R: Read + Seek> ReadFmt<Wav, WavBlocks> for BufReader<R> {
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

    #[inline]
    fn read_blocks(&mut self, wav: &Wav) -> Result<WavBlocks> {
        let blocks = wav.blocks();
        let mut wav_block_vec = Vec::<WavBlock>::with_capacity(blocks as usize);
        for _ in 0..blocks {
            let wav_block = self.read_block(wav)?;
            wav_block_vec.push(wav_block);
        }

        Ok(wav_block_vec.into_boxed_slice().into())
    }
}

/// This trait reads Bubbles in format for times.
pub trait ReadBubsIn<T, B>: Read {
    /// This method reads details of Bubbles in format for times.
    /// 
    /// # Examples
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use floaout::format::oao::{BubblesInFloaout, Floaout};
    /// use floaout::io::read::{ReadBubsIn, ReadFmt};
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut reader = io::BufReader::new(File::open("foo.oao")?);
    ///     let oao: Floaout = reader.read_details()?;
    /// 
    ///     // read BubbleInFloaout details for 2 times.
    ///     let bubs_in_oao: BubblesInFloaout = reader.read_bubs_details(&oao)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn read_bubs_details(&mut self, _: &T) -> Result<B>;
}

impl<R: Read + Seek> ReadBubsIn<Floaout, BubblesInFloaout> for BufReader<R> {
    #[inline]
    fn read_bubs_details(&mut self, oao: &Floaout) -> Result<BubblesInFloaout> {
        // Into Vec
        let mut vec_of_bub_in_oao: Vec<BubbleInFloaout> = Vec::new();
        for _ in 0..oao.bubbles {
            let name_size: u8 = self.read_le_bytes()?;
            vec_of_bub_in_oao.push(
                BubbleInFloaout {
                    name_size,
                    name: self.read_be_bytes_for(name_size as usize)?,
                    color: self.read_le_bytes()?
                }
            );
        }

        Ok(BubblesInFloaout::from(vec_of_bub_in_oao))
    }
}