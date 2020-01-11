use std::fs::{File, rename};
use std::io::{Read, BufReader, BufWriter, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


#[derive(Debug)]
pub struct Wav {
    // Riff Chunk
    riff: String,
    wav_size: u32,    // File Size-8
    wave: String,
    // Format Chunk
    format: String,
    format_size: u32,    // 16
    format_code: u16,    // 3
    channel: u16,    // Channel    1Channel
    sampling_rate: u32,    // Hz    44100Hz
    data_rate: u32,    // kbps    Bytes/Sec    44100*4*1=176400kbps
    data_block_size: u16,    // Bytes/Sample*Channel    4
    bits_per_sample: u16,    // Bits/Sample    32-bit
    // Data Chunk
    data: String,
    wave_size: u32,
    // Other Chunk
    other_size: u32,
}

impl Default for Wav {
    fn default() -> Self {
        Self {
            // Riff Chunk
            riff: String::from("RIFF"),
            wav_size: Default::default(),
            wave: String::from("WAVE"),
            // Format Chunk
            format: String::from("fmt "),
            format_size: Default::default(),
            format_code: Default::default(),
            channel: Default::default(),
            sampling_rate: Default::default(),
            data_rate: Default::default(),
            data_block_size: Default::default(),
            bits_per_sample: Default::default(),
            // Data Chunk
            data: String::from("data"),
            wave_size: Default::default(),
            // Other Chunk
            other_size: Default::default(),
        }
    }
}

impl Wav {


    pub fn file_unhead_reader(wav_file: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Skip Riff ~ Wave
        let mut skip = [0; 44];
        reader.read(&mut skip)?;

        Ok(reader)
    }

    pub fn wav_format(wav_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get infomation of Wav structure.
        let wav_info = Wav::from_file(wav_file)?;

        // Writer
        let mut new_file = String::from("format_");
        new_file.push_str(&wav_file);
        let mut writer = BufWriter::new(File::create(new_file)?);
        // Reader
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Repeat when there is a chunk.
        loop {
            let mut chunk_name = [0; 4];
            reader.read(&mut chunk_name)?;
            // Allocate by chunk name.
            match chunk_name {
                // RIFF
                [82, 73, 70, 70] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    // There is no other chunk in this wav_file.
                    let mut wav_size = reader.read_u32::<LittleEndian>()?;
                    // So we have to minus other chunk size.
                    wav_size -= wav_info.other_size;
                    writer.write_u32::<LittleEndian>(wav_size)?;
                    let mut wave = [0; 4];
                    reader.read(&mut wave)?;
                    writer.write(&wave)?;
                },
                // Format
                [102, 109, 116, 32] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    let mut format_size = [0; 4];
                    reader.read(&mut format_size)?;
                    writer.write(&format_size)?;
                    let mut format_code = [0; 2];
                    reader.read(&mut format_code)?;
                    writer.write(&format_code)?;
                    let mut channel = [0; 2];
                    reader.read(&mut channel)?;
                    writer.write(&channel)?;
                    let mut sampling_rate = [0; 4];
                    reader.read(&mut sampling_rate)?;
                    writer.write(&sampling_rate)?;
                    let mut data_rate = [0; 4];
                    reader.read(&mut data_rate)?;
                    writer.write(&data_rate)?;
                    let mut data_block_size = [0; 2];
                    reader.read(&mut data_block_size)?;
                    writer.write(&data_block_size)?;
                    let mut bits_per_sample = [0; 2];
                    reader.read(&mut bits_per_sample)?;
                    writer.write(&bits_per_sample)?;
                },
                // Data
                [100, 97, 116, 97] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    let wave_size = reader.read_u32::<LittleEndian>()?;
                    writer.write_u32::<LittleEndian>(wave_size)?;
                    let mut wave_data = [0; 4];
                    for _ in 0..wave_size / 4 {
                        reader.read(&mut wave_data)?;
                        writer.write(&wave_data)?;
                    }
                    break
                },
                // Other
                _ => {
                    let chunk_size = reader.read_u32::<LittleEndian>()?;
                    // skip for chunk size times.
                    let mut skip = [0; 1];
                    for _ in 0..chunk_size {
                        reader.read(&mut skip)?;
                    }
                },
            }
        }

        Ok(())
    }

    pub fn head_writer(self, mut writer: BufWriter<File>) -> Result<BufWriter<File>, Box<dyn std::error::Error>> {
        // Riff Chunk
        writer.write(&self.riff.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.wav_size)?;
        writer.write(&self.wave.into_bytes())?;
        // Format Chunk
        writer.write(&self.format.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.format_size)?;
        writer.write_u16::<LittleEndian>(self.format_code)?;
        writer.write_u16::<LittleEndian>(self.channel)?;
        writer.write_u32::<LittleEndian>(self.sampling_rate)?;
        writer.write_u32::<LittleEndian>(self.data_rate)?;
        writer.write_u16::<LittleEndian>(self.data_block_size)?;
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        // Data Chunk
        writer.write(&self.data.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.wave_size)?;

        Ok(writer)
    }

    pub fn with_channels(&self, channels: usize) -> Self {
        // Initialize
        let mut wav_info = Self {
            .. Default::default()
        };
        // Wav Size
        wav_info.wav_size = self.wave_size * channels as u32 + 36;
        // Format Size
        wav_info.format_size = self.format_size;
        // Format Code
        wav_info.format_code = self.format_code;
        // Channel
        wav_info.channel = channels as u16;
        // Sampling Rate
        wav_info.sampling_rate = self.sampling_rate;
        // Data Rate
        wav_info.data_rate = self.sampling_rate * self.bits_per_sample as u32 / 8 * channels as u32;
        // Data Block Size
        wav_info.data_block_size = self.bits_per_sample / 8 * channels as u16;
        // Bits Per Sample
        wav_info.bits_per_sample = self.bits_per_sample;
        // Wave Data Size
        wav_info.wave_size = self.wave_size * channels as u32;

        wav_info
    }

    pub fn wav_volume(wav_file: &str, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
        // Get infomation of Wav structure.
        let wav_info = Wav::from_file(wav_file)?;

        // writer
        let mut new_file = volume.to_string();
        new_file.push_str(&wav_file);
        let mut writer = BufWriter::new(File::create(new_file)?);
        // reader
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Repeat when there is a chunk.
        loop {
            let mut chunk_name = [0; 4];
            reader.read(&mut chunk_name)?;
            // Allocate by chunk name.
            match chunk_name {
                // RIFF
                [82, 73, 70, 70] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    // There is no other chunk in this wav_file.
                    let mut file_size = reader.read_u32::<LittleEndian>()?;
                    // So we have to minus other chunk size.
                    file_size -= wav_info.other_size;
                    writer.write_u32::<LittleEndian>(file_size)?;
                    let mut wave = [0; 4];
                    reader.read(&mut wave)?;
                    writer.write(&wave)?;
                },
                // Format
                [102, 109, 116, 32] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    let mut format_size = [0; 4];
                    reader.read(&mut format_size)?;
                    writer.write(&format_size)?;
                    let mut format_code = [0; 2];
                    reader.read(&mut format_code)?;
                    writer.write(&format_code)?;
                    let mut channel = [0; 2];
                    reader.read(&mut channel)?;
                    writer.write(&channel)?;
                    let mut sampling_rate = [0; 4];
                    reader.read(&mut sampling_rate)?;
                    writer.write(&sampling_rate)?;
                    let mut data_rate = [0; 4];
                    reader.read(&mut data_rate)?;
                    writer.write(&data_rate)?;
                    let mut data_block_size = [0; 2];
                    reader.read(&mut data_block_size)?;
                    writer.write(&data_block_size)?;
                    let mut bits_per_sample = [0; 2];
                    reader.read(&mut bits_per_sample)?;
                    writer.write(&bits_per_sample)?;
                },
                // Data
                [100, 97, 116, 97] => {
                    let chunk_name = chunk_name;
                    writer.write(&chunk_name)?;
                    let wave_size = reader.read_u32::<LittleEndian>()?;
                    writer.write_u32::<LittleEndian>(wave_size)?;
                    for _ in 0..wave_size / 4 {
                        let mut wave_data = reader.read_f32::<LittleEndian>()?;
                        // volume: u32 is [MIN:0, MAX:1]
                        wave_data *= volume;
                        writer.write_f32::<LittleEndian>(wave_data)?;
                    }
                    break
                },
                // Other
                _ => {
                    let chunk_size = reader.read_u32::<LittleEndian>()?;
                    // skip for chunk size times.
                    let mut skip = [0; 1];
                    for _ in 0..chunk_size {
                        reader.read(&mut skip)?;
                    }
                },
            }
        }

        Ok(())
    }


}


#[derive(Clone, Debug)]
pub struct Blower {
    // Blower
    pub blower: String,
    pub bubbles: u16,    // Number of Bubbles (0~65535)
    // Bubble Field
    pub length: u16,    // (0~65535)
    pub width: u16,    // (0~65535)
    pub height: u16,    // (0~65535)
    // Data Information
    pub samples: u64,    // Number of Sample
    pub sampling_rate: u32,    // Hz
    pub bits_per_sample: u16,    // Bit/Sample    32-bit
    // Audio Information
    pub title_size: u32,
    pub title: String,    // Title (UTF-8)
    pub artist_size: u32,
    pub artist: String,    // Artist (UTF-8)
    // Bubble Information
    pub bubbles_name_size: Vec<u32>,
    pub bubbles_name: Vec<String>,   // Bubbles Name
    pub bubbles_time_size: Vec<u64>,
    pub bubbles_time: Vec<(u64, u64)>,   // (start, end)
    pub format_code: u16,    // 3
}

impl Default for Blower {
    fn default() -> Self {
        Self {
            // Blower
            blower: String::from("blow"),
            bubbles: Default::default(),
            // Bubble Field
            length: Default::default(),
            width: Default::default(),
            height: Default::default(),
            // Data Information
            samples: Default::default(),
            sampling_rate: Default::default(),
            bits_per_sample: Default::default(),
            // Audio Information
            title_size: Default::default(),
            title: Default::default(),
            artist_size: Default::default(),
            artist: Default::default(),
            // Bubble Information
            bubbles_name_size: Default::default(),
            bubbles_name: Default::default(),
            bubbles_time_size: Default::default(),
            bubbles_time: Default::default(),
            format_code: Default::default(),            
        }
    }
}


impl Blower {


    pub fn pop(&self, positions: Vec<(usize, usize, usize)>) -> Result<(), Box<dyn std::error::Error>> {
        // Clone Blower Information
        let blower_info = self.clone();
        // Blower -> Wav Structure
        let wav_info = self.to_wav_info();
        // Channels
        let channels = positions.len();
        let new_wav_info = wav_info.with_channels(channels);
        // Make new Wav File
        // Writer
        let new_wav_file = format!("{}.wav", self.title);
        let mut writer = BufWriter::new(File::create(new_wav_file)?);
        let mut writer = new_wav_info.head_writer(writer)?;

        // Bubbles in BLower Information
        let bubbles_in_blower_info = BubblesInBlower::from_blower_info(blower_info);
        // Clone Bubbles in BLower Information
        let bubbles_in_blower_info_clone = bubbles_in_blower_info.clone();
        // Reader
        let mut reader: Vec<BufReader<File>> = Vec::new();
        for bubble_in_blower_info in bubbles_in_blower_info {
            // Bubble File
            let bubble_file = format!("{}.bub", bubble_in_blower_info.name);
            reader.push(Bubble::file_unhead_reader(&bubble_file)?);
        }
        // Writer
        // Wave Data
        let mut wave_data: Vec<f32> = vec![0.0; channels];
        // Bubble Field
        let mut length: usize;
        let mut width: usize;
        let mut height: usize;
        for sample in 0..self.samples {
            for bubble_in_blower_info in &bubbles_in_blower_info_clone {

                // Positions
                for position in &positions {
                    // Bubble Field
                    
                }
            }
            // Write Wave Data
            for i in 0..channels {
            writer.write_f32::<LittleEndian>(wave_data[i])?;                
            }
        }


        Ok(())
    }


}


#[derive(Clone, Debug)]
pub struct BubblesInBlower {
    // Bubble Information
    pub name: String,   // Bubbles Name
    pub time: Vec<(u64, u64)>,   // (start, end)
}

impl Default for BubblesInBlower {
    fn default() -> Self {
        Self {
            name: Default::default(),
            time: Default::default(),
        }
    }
}


impl BubblesInBlower {


    pub fn from_blower_info(mut blower_info: Blower) -> Vec<Self> {
        // Bubbles in Blower Information
        let mut bubbles_in_blower_info: Vec<Self> = Vec::new();
        for bubble_name in blower_info.bubbles_name.into_iter().rev() {
            // Initialize
            let mut info = Self {
                .. Default::default()
            };
            // Name
            info.name = bubble_name;
            // Number of Time
            let number_of_time = blower_info.bubbles_time_size.pop().unwrap() / 16;
            // Time
            for _ in 0..number_of_time {
                info.time.push(blower_info.bubbles_time.pop().unwrap());
            }
            // Push Bubbles in Blower Information
            bubbles_in_blower_info.push(info);
        }

        bubbles_in_blower_info
    }


}


#[derive(Debug)]
pub struct Bubble {
    // Bubble
    pub bubble: String,
    // Bubble Field
    pub length: u16,    // (0~65535)
    pub width: u16,    // (0~65535)
    pub height: u16,    // (0~65535)
    // Color
    pub red: u8,    // red
    pub green: u8,    // green
    pub blue: u8,    // Blue
    // Data Information
    pub samples: u64,    // Number of Sample
    pub sampling_rate: u32,    // Hz
    pub bits_per_sample: u16,    // Bit/Sample    32-bit
    // Bubble Information
    pub name_size: u32,
    pub name: String, // Name of bubble
    pub overall_size: u64,
    pub overall: Vec<Vec<Vec<f32>>>, // Overall of Bubble Field
    pub format_code: u16,    // 3
}

impl Default for Bubble {
    fn default() -> Self {
        Self {
            bubble: String::from("bub "),
            // Bubble Field
            length: Default::default(),
            width: Default::default(),
            height: Default::default(),
            red: Default::default(),
            green: Default::default(),
            blue: Default::default(),
            samples: Default::default(),
            sampling_rate: Default::default(),
            bits_per_sample: Default::default(),
            name_size: Default::default(),
            name: Default::default(),
            overall_size: Default::default(),
            overall: Default::default(),
            format_code: Default::default(),
        }
    }
}

impl Bubble {


    pub fn file_unhead_reader(bubble_file: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
        // Reader
        let mut reader = BufReader::new(File::open(bubble_file)?);

        // Skip Bubble ~ Bits Per Sample
        let mut skip = [0; 27];
        reader.read(&mut skip)?;

        // Name Size
        let name_size = reader.read_u32::<LittleEndian>()?;
        // Skip Name
        let mut skip = [0; 1];
        for _ in 0..name_size {
            reader.read(&mut skip)?;
        }
        // Overall Size
        let overall_size = reader.read_u64::<LittleEndian>()?;
        // Skip Overall
        let mut skip = [0; 1];
        for _ in 0..overall_size {
            reader.read(&mut skip)?;
        }
        // Skip Format Code
        let mut skip = [0; 2];
        reader.read(&mut skip)?;

        Ok(reader)
    }

    pub fn bubble_make(self, wav_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Writer
        let new_file = format!("{}.bub", self.name);
        let mut writer = BufWriter::new(File::create(new_file)?);
        // Write Bubble information.
        // Bubble
        writer.write(&self.bubble.into_bytes())?;
        // Bubble Field
        writer.write_u16::<LittleEndian>(self.length)?;
        writer.write_u16::<LittleEndian>(self.width)?;
        writer.write_u16::<LittleEndian>(self.height)?;
        // Color
        writer.write(&[self.red])?;
        writer.write(&[self.green])?;
        writer.write(&[self.blue])?;
        // Samples
        writer.write_u64::<LittleEndian>(self.samples)?;
        // Sampling Rate
        writer.write_u32::<LittleEndian>(self.sampling_rate)?;
        // Bits Per Sample
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        // Name
        writer.write_u32::<LittleEndian>(self.name_size)?;
        writer.write(&self.name.into_bytes())?;
        // Overall
        writer.write_u64::<LittleEndian>(self.overall_size)?;
        for height in 0..self.height as usize {
            for width in 0..self.width as usize {
                for length in 0..self.length as usize {
                    writer.write_f32::<LittleEndian>(self.overall[length][width][height])?;
                }
            }
        }
        // Format Code
        writer.write_u16::<LittleEndian>(self.format_code)?;
        // Data
        // Reader
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Skip Riff ~ Wave Size
        let mut skip = [0; 44];
        reader.read(&mut skip)?;
        // Data
        let mut wave_data = [0; 4];
        for _ in 0..self.samples {
            // Form n
            for height in 0..self.height as usize {
                for width in 0..self.width as usize {
                    for length in 0..self.length as usize {
                        writer.write_f32::<LittleEndian>(self.overall[length][width][height])?;
                    }
                }
            }
            // Wave Data n
            reader.read(&mut wave_data)?;
            writer.write(&wave_data)?;
        }


        Ok(())
    }

    pub fn bubble_to_wav(bubble_file: &str, length: usize, width: usize, height: usize) -> Result<(), Box<dyn std::error::Error>> {
        // Get information of Bubble structure.
        let bubble_info = Bubble::from_file(bubble_file)?;
        // Bubble structure to Wav structure
        let wav_info = bubble_info.to_wav_info();
        // Writer
        let mut wav_file = bubble_info.name;
        wav_file.push_str("_to.wav");
        let mut writer = BufWriter::new(File::create(wav_file)?);

        // Riff Chunk
        writer.write(&wav_info.riff.into_bytes())?;
        writer.write_u32::<LittleEndian>(wav_info.wav_size)?;
        writer.write(&wav_info.wave.into_bytes())?;
        // Format Chunk
        writer.write(&wav_info.format.into_bytes())?;
        writer.write_u32::<LittleEndian>(wav_info.format_size)?;
        writer.write_u16::<LittleEndian>(wav_info.format_code)?;
        writer.write_u16::<LittleEndian>(wav_info.channel)?;
        writer.write_u32::<LittleEndian>(wav_info.sampling_rate)?;
        writer.write_u32::<LittleEndian>(wav_info.data_rate)?;
        writer.write_u16::<LittleEndian>(wav_info.data_block_size)?;
        writer.write_u16::<LittleEndian>(wav_info.bits_per_sample)?;
        // Data Chunk
        writer.write(&wav_info.data.into_bytes())?;
        writer.write_u32::<LittleEndian>(wav_info.wave_size)?;
        // Reader
        let mut reader = Bubble::file_unhead_reader(bubble_file)?;
        // Wave Data
        let mut form: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; bubble_info.height as usize]; bubble_info.width as usize]; bubble_info.length as usize];
        for _ in 0..bubble_info.samples {
            // Form
            for height in 0..bubble_info.height as usize {
                for width in 0..bubble_info.width as usize {
                    for length in 0..bubble_info.length as usize {
                        form[length][width][height] = reader.read_f32::<LittleEndian>()?;
                    }
                }
            }
            // Wave Data
            let mut wave_data = reader.read_f32::<LittleEndian>()?;
            // Volume
            wave_data *= form[length][width][height];
            writer.write_f32::<LittleEndian>(wave_data)?;
        }

        Ok(())
    }

    pub fn bubble_in_wav(wav_file: &str, bubble_file: &str, length: usize, width: usize, height: usize, bubble_time: (u64, u64)) -> Result<(), Box<dyn std::error::Error>> {
        // Information
        let wav_info = Wav::from_file(wav_file)?;
        let bubble_info = Bubble::from_file(bubble_file)?;
        // Writer
        let mut writer = BufWriter::new(File::create("bubble_in.wav")?);
        // Reader
        let mut wav_reader = BufReader::new(File::open(wav_file)?);
        let mut bubble_reader = Bubble::file_unhead_reader(bubble_file)?;
        // Bubble Time
        let (start, end) = bubble_time;
        // Write Header
        let mut header = [0; 44];
        wav_reader.read(&mut header)?;
        writer.write(&header)?;
        // Wave Data
        let mut wav_wave_data: f32;
        let mut bubble_wave_data: f32;
        let mut form: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; bubble_info.height as usize]; bubble_info.width as usize]; bubble_info.length as usize];
        for sample in 0..wav_info.wave_size as u64 / wav_info.bits_per_sample as u64 * 8 {
            if start <= sample && sample <= end {
                // Form
                for height in 0..bubble_info.height as usize {
                    for width in 0..bubble_info.width as usize {
                        for length in 0..bubble_info.length as usize {
                            form[length][width][height] = bubble_reader.read_f32::<LittleEndian>()?;
                        }
                    }
                }
                // Wave Data
                bubble_wave_data = bubble_reader.read_f32::<LittleEndian>()?;
                // Volume
                bubble_wave_data *= form[length][width][height];
                wav_wave_data = wav_reader.read_f32::<LittleEndian>()?;
                wav_wave_data += bubble_wave_data;
                writer.write_f32::<LittleEndian>(wav_wave_data)?;
            } else {
                // Wave Data
                wav_wave_data = wav_reader.read_f32::<LittleEndian>()?;
                writer.write_f32::<LittleEndian>(wav_wave_data)?;
            }
        }
        // Rename
        rename("bubble_in.wav", wav_file)?;

        Ok(())
    }


}


pub trait Extensions {


    // Information
    fn info_print(&self);

    // Structure -> Structure
    fn to_wav_info(&self) -> Wav;

    // File -> Structure
    fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> where Self: std::marker::Sized;
    fn from_wav_file(wav_file: &str) -> Result<Self, Box<dyn std::error::Error>> where Self: std::marker::Sized;

    // Structure -> File
    fn make_file(self) -> Result<(), Box<dyn std::error::Error>>;


}


impl Extensions for Wav {


    fn info_print(&self) {
        // Print the wav file informatiion.
        println!("\nThis is the wav file information!!\n");
        println!(" File Size( - 8 ): {} Bytes", self.wav_size);
        println!("Format Chunk Size: {} Bytes", self.format_size);
        println!("      Format Code: {}", self.format_code);
        println!("          Channel: {} Channels", self.channel);
        println!("    Sampling Rate: {} Hz", self.sampling_rate);
        println!("        Data Rate: {} kbps", self.data_rate);
        println!("  Data Block Size: {} Bytes", self.data_block_size);
        println!("  Bits Per Sample: {} Bits", self.bits_per_sample);
        println!("   Wave Data Size: {} Bytes", self.wave_size);
        println!(" Other Chunk Size: {} Bytes", self.other_size);
    }

    // Does NOT WORK
    fn to_wav_info(&self) -> Wav {
        let  wav_info = Wav {
            .. Default::default()
        };

        wav_info
    }
    fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize
        let mut wav_info = Self {
            .. Default::default()
        };

        // Reader
        let mut reader = BufReader::new(File::open(file)?);
        // Count bytes of other chunk.
        let mut other_size = 0;
        // Repeat when there is a chunk.
        loop {
            let mut chunk_name = [0; 4];
            reader.read(&mut chunk_name)?;
            // Allocate by chunk name.
            match chunk_name {
                // RIFF
                [82, 73, 70, 70] => {
                    let wav_size = reader.read_u32::<LittleEndian>()?;
                    wav_info.wav_size = wav_size;
                    let mut wave = [0; 4];
                    reader.read(&mut wave)?;
                },
                // Format
                [102, 109, 116, 32] => {
                    let format_size = reader.read_u32::<LittleEndian>()?;
                    wav_info.format_size = format_size;
                    let format_code = reader.read_u16::<LittleEndian>()?;
                    wav_info.format_code = format_code;
                    let channel = reader.read_u16::<LittleEndian>()?;
                    wav_info.channel = channel;
                    let sampling_rate = reader.read_u32::<LittleEndian>()?;
                    wav_info.sampling_rate = sampling_rate;
                    let data_rate = reader.read_u32::<LittleEndian>()?;
                    wav_info.data_rate = data_rate;
                    let data_block_size = reader.read_u16::<LittleEndian>()?;
                    wav_info.data_block_size = data_block_size;
                    let bits_per_sample = reader.read_u16::<LittleEndian>()?;
                    wav_info.bits_per_sample = bits_per_sample;
                },
                // Data
                [100, 97, 116, 97] => {
                    let wave_size = reader.read_u32::<LittleEndian>()?;
                    wav_info.wave_size = wave_size;
                    break
                },
                // Other
                _ => {
                    let chunk_size = reader.read_u32::<LittleEndian>()?;
                    // Add 8 and chunk_size bytes to other_size.
                    other_size += 8 + chunk_size;
                    let mut skip = [0; 1];
                    for _ in 0..chunk_size {
                        reader.read(&mut skip)?;
                    }
                },
            }
        }
        // Other Chunk Size
        wav_info.other_size = other_size;

        Ok(wav_info)
    }

    fn from_wav_file(wav_file: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let wav_info = Wav::from_file(wav_file)?;

        Ok(wav_info)
    }

    fn make_file(self) -> Result<(), Box<dyn std::error::Error>> {
        // Writer
        let  wav_file = "new.wav";
        let mut writer = BufWriter::new(File::create(wav_file)?);

        // Riff Chunk
        writer.write(&self.riff.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.wav_size)?;
        writer.write(&self.wave.into_bytes())?;
        // Format Chunk
        writer.write(&self.format.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.format_size)?;
        writer.write_u16::<LittleEndian>(self.format_code)?;
        writer.write_u16::<LittleEndian>(self.channel)?;
        writer.write_u32::<LittleEndian>(self.sampling_rate)?;
        writer.write_u32::<LittleEndian>(self.data_rate)?;
        writer.write_u16::<LittleEndian>(self.data_block_size)?;
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        // Data Chunk
        writer.write(&self.data.into_bytes())?;
        writer.write_u32::<LittleEndian>(self.wave_size)?;
        let zero = [0; 1];
        for _ in 0..self.wave_size {
            writer.write(&zero)?;
        }

        Ok(())
    }


}


impl Extensions for Blower {


    fn info_print(&self) {
        // Print the Blower file informatiion.
        println!("\nThis is the Blower file information!!\n");
        println!("          Bubbles: {}", self.bubbles);
        println!("           Length: {}", self.length);
        println!("            Width: {}", self.width);
        println!("           Height: {}", self.height);
        println!("          Samples: {}", self.samples);
        println!("    Sampling Rate: {} Hz", self.sampling_rate);
        println!("  Bits Per Sample: {} Bits", self.bits_per_sample);
        println!("       Title Size: {} Bytes", self.title_size);
        println!("            Title: {}", self.title);
        println!("      Artist Size: {} Bytes", self.artist_size);
        println!("           Artist: {}", self.artist);
        println!("Bubbles Name Size: {:?} Bytes", self.bubbles_name_size);
        println!("     Bubbles Name: {:?}", self.bubbles_name);
        println!("Bubbles Time Size: {:?} Bytes", self.bubbles_time_size);
        println!("     Bubbles Time: {:?}", self.bubbles_time);
        println!("      Format Code: {}", self.format_code);
    }

    fn to_wav_info(&self) -> Wav {
        // Initialize
        let mut wav_info = Wav {
            .. Default::default()
        };
        // Wav Size
        // 36 + Wave Data
        wav_info.wav_size = 36 + self.samples as u32 * self.bits_per_sample as u32 / 8;
        // Format Size
        wav_info.format_size = 16;
        // Format Code
        wav_info.format_code = self.format_code;
        // Channel
        wav_info.channel = 1;
        // Sampling Rate
        wav_info.sampling_rate = self.sampling_rate;
        // Data Rate
        wav_info.data_rate = self.sampling_rate * self.bits_per_sample as u32 / 8 * 1;
        // Data Block Size
        wav_info.data_block_size = self.bits_per_sample / 8 * 1;
        // Bits Per Sample
        wav_info.bits_per_sample = self.bits_per_sample;
        // Wave Size
        wav_info.wave_size = self.samples as u32 * self.bits_per_sample as u32 / 8 * 1;

        wav_info
    }

    fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize
        let mut blower_info = Self {
            .. Default::default()
        };

        // Reader
        let mut reader = BufReader::new(File::open(file)?);

        // Blower
        let mut blower = [0; 4];
        reader.read(&mut blower)?;
        // Bubbles
        let bubbles = reader.read_u16::<LittleEndian>()?;
        blower_info.bubbles = bubbles;
        // Bubble Field
        // Length
        let length = reader.read_u16::<LittleEndian>()?;
        blower_info.length = length;
        // Width
        let width = reader.read_u16::<LittleEndian>()?;
        blower_info.width = width;
        // Height
        let height = reader.read_u16::<LittleEndian>()?;
        blower_info.height = height;
        // Samples
        let samples = reader.read_u64::<LittleEndian>()?;
        blower_info.samples = samples;
        // Sampling Rate
        let sampling_rate = reader.read_u32::<LittleEndian>()?;
        blower_info.sampling_rate = sampling_rate;
        // Bits Per Sample
        let bits_per_sample = reader.read_u16::<LittleEndian>()?;
        blower_info.bits_per_sample = bits_per_sample;
        // Title Size
        let title_size = reader.read_u32::<LittleEndian>()?;
        blower_info.title_size = title_size;
        // Title
        let mut u_8 = [0; 1];
        let mut title: Vec<u8> = Vec::new();
        for _ in 0..title_size {
            reader.read(&mut u_8)?;
            title.push(u_8[0]);
        }
        blower_info.title = String::from_utf8(title)?;
        // Artist Size
        let artist_size = reader.read_u32::<LittleEndian>()?;
        blower_info.artist_size = artist_size;
        // Artist
        let mut u_8 = [0; 1];
        let mut artist: Vec<u8> = Vec::new();
        for _ in 0..artist_size {
            reader.read(&mut u_8)?;
            artist.push(u_8[0]);
        }
        blower_info.artist = String::from_utf8(artist)?;
        // Bubbles Name Size
        let mut bubbles_name_size: Vec<u32> = Vec::new();
        for _ in 0..bubbles {
            let u_32 = reader.read_u32::<LittleEndian>()?;
            bubbles_name_size.push(u_32);
        }
        blower_info.bubbles_name_size = bubbles_name_size;
        // Bubbles Name
        let mut bubbles_name: Vec<String> = Vec::new();
        for bubble_name_size in &blower_info.bubbles_name_size {
            let mut u_8 = [0; 1];
            let mut bubble_name: Vec<u8> = Vec::new();
            for _ in 0..*bubble_name_size {
                reader.read(&mut u_8)?;
                bubble_name.push(u_8[0]);
            }
            bubbles_name.push(String::from_utf8(bubble_name)?);            
        }
        blower_info.bubbles_name = bubbles_name;
        // Bubbles Time Size
        let mut bubbles_time_size: Vec<u64> = Vec::new();
        for _ in 0..bubbles {
            let u_64 = reader.read_u64::<LittleEndian>()?;
            bubbles_time_size.push(u_64);
        }
        blower_info.bubbles_time_size = bubbles_time_size;
        // Bubbles Time
        let mut bubbles_time: Vec<(u64, u64)> = Vec::new();
        for bubble_time_size in &blower_info.bubbles_time_size {
            for _ in 0..*bubble_time_size / 16 {
                let start = reader.read_u64::<LittleEndian>()?;
                let end = reader.read_u64::<LittleEndian>()?;
                let bubble_time = (start, end);
                bubbles_time.push(bubble_time);
            }
        }
        blower_info.bubbles_time = bubbles_time;
        // Format Code
        let format_code = reader.read_u16::<LittleEndian>()?;
        blower_info.format_code = format_code;        

        Ok(blower_info)
    }

    fn from_wav_file(wav_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize
        let mut blower_info = Self {
            .. Default::default()
        };
        // Reader
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Skip Riff ~ Format Size
        let mut skip = [0; 20];
        reader.read(&mut skip)?;
        // Format Code
        let format_code = reader.read_u16::<LittleEndian>()?;
        blower_info.format_code = format_code;
        // Skip Channel
        let mut skip = [0; 2];
        reader.read(&mut skip)?;
        // Sampling Rate
        let sampling_rate = reader.read_u32::<LittleEndian>()?;
        blower_info.sampling_rate = sampling_rate;
        // Skip Data Rate ~ Data Block Size
        let mut skip = [0; 6];
        reader.read(&mut skip)?;
        // Bits Per Sample
        let bits_per_sample = reader.read_u16::<LittleEndian>()?;
        blower_info.bits_per_sample = bits_per_sample;
        // Skip Data
        let mut skip = [0; 4];
        reader.read(&mut skip)?;
        // Wave Size
        let wave_size = reader.read_u32::<LittleEndian>()?;
        blower_info.samples = wave_size as u64 / bits_per_sample as u64 * 8;

        Ok(blower_info)
    }

    fn make_file(self) -> Result<(), Box<dyn std::error::Error>> {
        // Writer
        let new_file = format!("new_{}.blow", self.title); 
        let mut writer = BufWriter::new(File::create(new_file)?);
        // Write Blower information.
        // Blower
        writer.write(&self.blower.into_bytes())?;
        // Bubbles
        writer.write_u16::<LittleEndian>(self.bubbles)?;
        // Bubble Field
        writer.write_u16::<LittleEndian>(self.length)?;
        writer.write_u16::<LittleEndian>(self.width)?;
        writer.write_u16::<LittleEndian>(self.height)?;
        // Samples
        writer.write_u64::<LittleEndian>(self.samples)?;
        // Sampling Rate
        writer.write_u32::<LittleEndian>(self.sampling_rate)?;
        // Bits Per Sample
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        // Title
        writer.write_u32::<LittleEndian>(self.title_size)?;
        writer.write(&self.title.into_bytes())?;
        // Artist
        writer.write_u32::<LittleEndian>(self.artist_size)?;
        writer.write(&self.artist.into_bytes())?;
        // Bubbles Name
        for bubble_name_size in self.bubbles_name_size {
            writer.write_u32::<LittleEndian>(bubble_name_size)?;            
        }
        for bubble_name in self.bubbles_name {
            writer.write(&bubble_name.into_bytes())?;           
        }
        // Bubbles Time
        for bubble_time_size in self.bubbles_time_size {
            writer.write_u64::<LittleEndian>(bubble_time_size)?;            
        }
        for bubble_time in self.bubbles_time {
            let (start, end) = bubble_time;
            writer.write_u64::<LittleEndian>(start)?;
            writer.write_u64::<LittleEndian>(end)?;
        }
        // Format Code
        writer.write_u16::<LittleEndian>(self.format_code)?;

        Ok(())
    }


}


impl Extensions for Bubble {


    fn info_print(&self) {
        // Print the Bubble file informatiion.
        println!("\nThis is the Bubble file information!!\n");
        println!("           Length: {}", self.length);
        println!("            Width: {}", self.width);
        println!("           Height: {}", self.height);
        println!("              Red: {}", self.red);
        println!("            Green: {}", self.green);
        println!("             Blue: {}", self.blue);
        println!("          Samples: {}", self.samples);
        println!("    Sampling Rate: {} Hz", self.sampling_rate);
        println!("  Bits Per Sample: {} Bits", self.bits_per_sample);
        println!("        Name Size: {} Bytes", self.name_size);
        println!("             Name: {}", self.name);
        println!("     Overall Size: {} Bytes", self.overall_size);
        println!("          Overall: {:?}", self.overall);
        println!("      Format Code: {}", self.format_code);
    }

    fn to_wav_info(&self) -> Wav {
        // Initialize
        let mut wav_info = Wav {
            .. Default::default()
        };
        // Wav Size
        // 36 + Wave Data
        wav_info.wav_size = 36 + self.samples as u32 * self.bits_per_sample as u32 / 8;
        // Format Size
        wav_info.format_size = 16;
        // Format Code
        wav_info.format_code = self.format_code;
        // Channel
        wav_info.channel = 1;
        // Sampling Rate
        wav_info.sampling_rate = self.sampling_rate;
        // Data Rate
        wav_info.data_rate = self.sampling_rate * self.bits_per_sample as u32 / 8 * 1;
        // Data Block Size
        wav_info.data_block_size = self.bits_per_sample / 8 * 1;
        // Bits Per Sample
        wav_info.bits_per_sample = self.bits_per_sample;
        // Wave Size
        wav_info.wave_size = self.samples as u32 * self.bits_per_sample as u32 / 8 * 1;

        wav_info
    }

    fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize
        let mut bubble_info = Self {
            .. Default::default()
        };

        // Reader
        let mut reader = BufReader::new(File::open(file)?);

        // Bubble
        let mut bubble = [0; 4];
        reader.read(&mut bubble)?;
        // Bubble Field
        // Length
        let length = reader.read_u16::<LittleEndian>()?;
        bubble_info.length = length;
        // Width
        let width = reader.read_u16::<LittleEndian>()?;
        bubble_info.width = width;
        // Height
        let height = reader.read_u16::<LittleEndian>()?;
        bubble_info.height = height;
        // Color
        let mut color = [0; 1];
        // Red
        reader.read(&mut color)?;
        bubble_info.red = color[0];
        // Green
        reader.read(&mut color)?;
        bubble_info.green = color[0];
        // Blue
        reader.read(&mut color)?;
        bubble_info.blue = color[0];
        // Samples
        let samples = reader.read_u64::<LittleEndian>()?;
        bubble_info.samples = samples;
        // Sampling Rate
        let sampling_rate = reader.read_u32::<LittleEndian>()?;
        bubble_info.sampling_rate = sampling_rate;
        // Bits Per Sample
        let bits_per_sample = reader.read_u16::<LittleEndian>()?;
        bubble_info.bits_per_sample = bits_per_sample;
        // Name Size
        let name_size = reader.read_u32::<LittleEndian>()?;
        bubble_info.name_size = name_size;
        // Name
        let mut u_8 = [0; 1];
        let mut name: Vec<u8> = Vec::new();
        for _ in 0..name_size {
            reader.read(&mut u_8)?;
            name.push(u_8[0]);
        }
        bubble_info.name = String::from_utf8(name)?;
        // Overall Size
        let overall_size = reader.read_u64::<LittleEndian>()?;
        bubble_info.overall_size = overall_size;
        // Overall
        let mut overall: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; height as usize]; width as usize]; length as usize];
        for height  in 0..height as usize {
            for width in 0..width as usize {
                for length in 0..length as usize {
                    overall[length][width][height] = reader.read_f32::<LittleEndian>()?;
                }
            }
        }
        bubble_info.overall = overall;
        // Format Code
        let format_code = reader.read_u16::<LittleEndian>()?;
        bubble_info.format_code = format_code;

        Ok(bubble_info)
    }

    fn from_wav_file(wav_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize
        let mut bubble_info = Self {
            .. Default::default()
        };
        // Reader
        let mut reader = BufReader::new(File::open(wav_file)?);
        // Skip Riff ~ Format Size
        let mut skip = [0; 20];
        reader.read(&mut skip)?;
        // Format Code
        let format_code = reader.read_u16::<LittleEndian>()?;
        bubble_info.format_code = format_code;
        // Skip Channel
        let mut skip = [0; 2];
        reader.read(&mut skip)?;
        // Sampling Rate
        let sampling_rate = reader.read_u32::<LittleEndian>()?;
        bubble_info.sampling_rate = sampling_rate;
        // Skip Data Rate ~ Data Block Size
        let mut skip = [0; 6];
        reader.read(&mut skip)?;
        // Bits Per Sample
        let bits_per_sample = reader.read_u16::<LittleEndian>()?;
        bubble_info.bits_per_sample = bits_per_sample;
        // Skip Data
        let mut skip = [0; 4];
        reader.read(&mut skip)?;
        // Wave Size
        let wave_size = reader.read_u32::<LittleEndian>()?;
        bubble_info.samples = wave_size as u64 / bits_per_sample as u64 * 8;

        Ok(bubble_info)
    }

    fn make_file(self) -> Result<(), Box<dyn std::error::Error>> {
        // Writer
        let new_file = format!("new_{}.bub", self.name); 
        let mut writer = BufWriter::new(File::create(new_file)?);
        // Write Bubble information.
        // Bubble
        writer.write(&self.bubble.into_bytes())?;
        // Bubble Field
        writer.write_u16::<LittleEndian>(self.length)?;
        writer.write_u16::<LittleEndian>(self.width)?;
        writer.write_u16::<LittleEndian>(self.height)?;
        // Color
        writer.write(&[self.red])?;
        writer.write(&[self.green])?;
        writer.write(&[self.blue])?;
        // Samples
        writer.write_u64::<LittleEndian>(self.samples)?;
        // Sampling Rate
        writer.write_u32::<LittleEndian>(self.sampling_rate)?;
        // Bits Per Sample
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        // Name
        writer.write_u32::<LittleEndian>(self.name_size)?;
        writer.write(&self.name.into_bytes())?;
        // Overall
        writer.write_u64::<LittleEndian>(self.overall_size)?;
        for height in 0..self.height as usize {
            for width in 0..self.width as usize {
                for length in 0..self.length as usize {
                    writer.write_f32::<LittleEndian>(self.overall[length][width][height])?;
                }
            }
        }
        // Format Code
        writer.write_u16::<LittleEndian>(self.format_code)?;
        // Data
        let zero = [0; 1];
        for _ in 0..self.samples {
            // Form n
            for _ in 0..self.height as usize {
                for _ in 0..self.width as usize {
                    for _ in 0..self.length as usize {
                        writer.write_f32::<LittleEndian>(0.0)?;
                    }
                }
            }
            // Wave Data n
            for _ in 0..self.bits_per_sample / 8 {
                writer.write(&zero)?;   
            }
        }


        Ok(())
    }


}