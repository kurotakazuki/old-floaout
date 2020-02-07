use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::wav::{Wav, WavBlock, WavBlocks};
use floaout::io::read::ReadFmt;
use floaout::io::write::WriteFmt;

#[test]
fn wav_details_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test1.wav";
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
    writer.write_details(&write_wav)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_wav: Wav = reader.read_details()?;

    assert_eq!(read_wav, write_wav);

    remove_file(file)?;

    Ok(())
}

#[test]
fn wav_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test2.wav";
    // Details of Wav that is going to be written.
    let write_wav = Wav {
        riff_size: 44,
        format_size: 16,
        format_tag: 3,
        channels: 1,
        sampling_rate: 44100,
        data_rate: 176400,
        data_block_size: 4,
        bits_per_sample: 32,
        data_size: 8,
        other_size: 0
    };
    // Wav blocks
    let wav_block1 = WavBlock::from(1.0f32);
    let wav_block2 = WavBlock::from(0.5f32);
    let wav_block_for_2 = vec![wav_block1, wav_block2];
    let write_wav_blocks = WavBlocks::from(wav_block_for_2.into_boxed_slice());
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(&write_wav)?;
    writer.write_blocks(&write_wav, write_wav_blocks.clone())?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_wav: Wav = reader.read_details()?;
    let read_wav_blocks: WavBlocks = reader.read_blocks(&read_wav)?;

    assert_eq!(read_wav, write_wav);
    assert_eq!(read_wav_blocks, write_wav_blocks);

    remove_file(file)?;

    Ok(())
}