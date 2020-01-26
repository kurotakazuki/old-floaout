use crate::format::wav::Wav;
use std::io;

pub trait ReadBytes<T>: io::Read {
    fn read_be_bytes(&mut self) -> io::Result<T>;
    fn read_le_bytes(&mut self) -> io::Result<T>;
}

// Maybe macro is better to write.

impl<R: io::Read + ?Sized> ReadBytes<f32> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<f32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<f32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }
}

impl<R: io::Read + ?Sized> ReadBytes<f64> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }
}

impl<R: io::Read + ?Sized> ReadBytes<u8> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }
}

impl<R: io::Read + ?Sized> ReadBytes<u16> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }
}

impl<R: io::Read + ?Sized> ReadBytes<u32> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }
}


impl<R: io::Read + ?Sized> ReadBytes<u64> for R {
    #[inline]
    fn read_be_bytes(&mut self) -> io::Result<u64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }

    #[inline]
    fn read_le_bytes(&mut self) -> io::Result<u64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

pub trait ReadExt<T>: io::Read {
    fn read_details(&mut self) -> io::Result<T>;
}

impl<R: io::Read + ?Sized> ReadExt<Wav> for R {
    #[inline]
    fn read_details(&mut self) -> io::Result<Wav> {
        let mut details = Wav::default();
        // Repeat when there is a chunk.
        loop {
            let mut chunk_name = [0; 4];
            self.read_exact(&mut chunk_name)?;
            // Allocate by chunk name.
            match chunk_name {
                // RIFF
                [82, 73, 70, 70] => {
                    details.riff_size = self.read_le_bytes()?;
                    let mut wave = [0; 4];
                    self.read_exact(&mut wave)?;
                },
                // Format
                [102, 109, 116, 32] => {
                    details.format_size = self.read_le_bytes()?;
                    details.format_code = self.read_le_bytes()?;
                    details.channels = self.read_le_bytes()?;
                    details.sampling_rate = self.read_le_bytes()?;
                    details.data_rate = self.read_le_bytes()?;
                    details.data_block_size = self.read_le_bytes()?;
                    details.bits_per_sample = self.read_le_bytes()?;
                },
                // Data
                [100, 97, 116, 97] => {
                    details.data_size = self.read_le_bytes()?;
                    break
                },
                // Other
                _ => {
                    let chunk_size: u32 = self.read_le_bytes()?;
                    // Add 8 and chunk_size bytes to other_size.
                    details.other_size += 8 + chunk_size;
                    let mut skip = [0; 1];
                    for _ in 0..chunk_size {
                        self.read_exact(&mut skip)?;
                    }
                },
            }
        }

        Ok(details)
    }
}