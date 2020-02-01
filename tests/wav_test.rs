extern crate floaout;

use std::io::{BufReader, BufWriter};
use std::fs::File;
use floaout::format::wav::Wav;
use floaout::io::read::ReadExt;
use floaout::io::write::WriteExt;

#[test]
fn wav_details_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test.wav";
    // Details of Wav that is going to be written.
    let write_wav = Wav {
        riff_size: 36,
        format_size: 16,
        format_tag: 3,
        channels: 1,
        sampling_rate: 44100,
        data_rate: 176400,
        data_block_size: 4,
        bits_per_sample: 32,
        data_size: 0,
        other_size: 0
    };
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(write_wav)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_wav: Wav = reader.read_details()?;

    assert_eq!(write_wav, read_wav);

    Ok(())
}