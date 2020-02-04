//READ

impl<R: Read + Seek> ReadFmt<Blower> for BufReader<R> {
    #[inline]
    fn read_details(&mut self) -> Result<Blower> {
        // Initialized
        let mut blow = Blower::default();
        // Blower
        read_assert_eq(self, "blow")?;
        blow.version = self.read_le_bytes()?;
        // Bubble field size
        blow.bub_field_size = self.read_le_bytes()?;
        // Format
        blow.bubbles = self.read_le_bytes()?;
        blow.blocks = self.read_le_bytes()?;
        blow.sampling_rate = self.read_le_bytes()?;
        blow.bits_per_sample = self.read_le_bytes()?;

        Ok(blow)
    }
}

impl<R: Read + Seek> ReadBubsIn<BubblesInBlower> for BufReader<R> {
    #[inline]
    fn read_bubs_details_for(&mut self, times: usize) -> Result<BubblesInBlower> {
        // Into Vec
        let mut vec_of_bub_in_blow: Vec<BubbleInBlower> = Vec::new();
        for _ in 0..times {
            // Name of Bubble
            let name_size: u8 = self.read_le_bytes()?;
            let name = self.read_be_bytes_for(name_size as usize)?;
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



// WRITE

impl<W: Write> WriteFmt<Blower, FloaoutBlocks> for BufWriter<W> {
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
fn write_blocks(&mut self, _: T, _: B) -> std::result::Result<(), std::io::Error> { unimplemented!() }
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