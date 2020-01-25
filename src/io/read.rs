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