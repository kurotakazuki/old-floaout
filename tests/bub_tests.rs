use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::BubbleField;
use floaout::format::bub::{Bubble, BubbleBlock, BubbleBlocks};
use floaout::format::wav::WavBlock;
use floaout::io::read::ReadFmt;
use floaout::io::write::WriteFmt;

#[test]
fn bub_details_without_name_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test1.bub";
    // Details of Bubble that is going to be written.
    let write_bub = Bubble {
        version: 0,
        bub_field_size: (1u8, 1u8, 1u8).into(),
        color: (0, 0, 0).into(),
        blocks: 0,
        sampling_rate: 44100,
        bits_per_sample: 32,
        name_size: 0,
        name: "".into(),
        overall: vec![vec![vec![0, 1], vec![2, 3]], vec![vec![4, 5], vec![6, 7]]].into()
    };
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(&write_bub)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_bub: Bubble = reader.read_details()?;

    assert_eq!(read_bub, write_bub);

    remove_file(file)?;

    Ok(())
}

#[test]
fn bub_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test2.bub";
    // Details of Bubble that is going to be written.
    let write_bub = Bubble {
        version: 0,
        bub_field_size: (2u8, 1u8, 0u8).into(),
        color: (255, 255, 255).into(),
        blocks: 2,
        sampling_rate: 44100,
        bits_per_sample: 32,
        name_size: 3,
        name: "た".into(),
        overall: vec![vec![vec![0], vec![4]], vec![vec![1], vec![5]], vec![vec![2], vec![6]], vec![vec![3], vec![7]]].into()
    };
    // Bubble block
    let wav_block1 = WavBlock::from(1.0f32);
    let bub_field_1: BubbleField = vec![vec![vec![0], vec![4]], vec![vec![1], vec![5]], vec![vec![2], vec![6]], vec![vec![3], vec![7]]].into();
    let wav_block2 = WavBlock::from(0.5f32);
    let bub_field_2: BubbleField = vec![vec![vec![255], vec![255]], vec![vec![255], vec![255]], vec![vec![255], vec![255]], vec![vec![255], vec![255]]].into();
    let bub_block_1 = BubbleBlock::from_wav_block_and_bub_field(wav_block1, bub_field_1);
    let bub_block_2 = BubbleBlock::from_wav_block_and_bub_field(wav_block2, bub_field_2);
    // Bubble blocks
    let bub_block_vec = vec![bub_block_1, bub_block_2];
    let write_bub_blocks = BubbleBlocks::from(bub_block_vec.into_boxed_slice());
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(&write_bub)?;
    writer.write_blocks(&write_bub, write_bub_blocks.clone())?;

    // Finish writing.
    drop(writer);

    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_bub: Bubble = reader.read_details()?;
    let read_bub_blocks: BubbleBlocks = reader.read_blocks(&read_bub)?;

    assert_eq!(read_bub, write_bub);
    assert_eq!(read_bub_blocks, write_bub_blocks);

    remove_file(file)?;

    Ok(())
}