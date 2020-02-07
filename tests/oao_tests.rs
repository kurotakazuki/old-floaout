use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::BubbleField;
use floaout::format::bub::BubbleBlock;
use floaout::format::oao::{Floaout, BubbleInFloaout, BubblesInFloaout, FloaoutBlock, FloaoutBlocks};
use floaout::format::wav::WavBlock;
use floaout::io::read::{ReadBubsIn, ReadFmt};
use floaout::io::write::{WriteBubsIn, WriteFmt};
#[test]
fn oao_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test.oao";
    // Details of Floaout that is going to be written.
    let write_oao = Floaout {
        version: 0,
        song_id: 0,
        bub_field_size: (2u8, 1u8, 0u8).into(),
        bubbles: 2,
        blocks: 2,
        sampling_rate: 44100,
        bits_per_sample: 32
    };
    // Details of BubblesInFloaout
    let write_bub1_in_oao = BubbleInFloaout {
        name_size: 3,
        name: "た".into(),
        color: (255, 255, 255).into()
    };
    let write_bub2_in_oao = BubbleInFloaout {
        name_size: 0,
        name: "".into(),
        color: (0, 0, 0).into()
    };
    let write_bubs_in_oao: BubblesInFloaout = vec!(write_bub1_in_oao, write_bub2_in_oao).into();

    // Bubble 1 block (name = "た")
    let wav1_block1 = WavBlock::from(1.0f32);
    let bub1_field_1: BubbleField = vec![vec![vec![0], vec![4]], vec![vec![1], vec![5]], vec![vec![2], vec![6]], vec![vec![3], vec![7]]].into();
    let wav1_block2 = WavBlock::from(0.5f32);
    let bub1_field_2: BubbleField = vec![vec![vec![255], vec![255]], vec![vec![255], vec![255]], vec![vec![255], vec![255]], vec![vec![255], vec![255]]].into();
    let bub1_block_1 = BubbleBlock::from_wav_block_and_bub_field(wav1_block1, bub1_field_1);
    let bub1_block_2 = BubbleBlock::from_wav_block_and_bub_field(wav1_block2, bub1_field_2);

    // Bubble 2 block (name = "")
    let wav2_block1 = WavBlock::from(0.25f32);
    let bub2_field_1: BubbleField = vec![vec![vec![1], vec![1]], vec![vec![1], vec![1]], vec![vec![1], vec![1]], vec![vec![1], vec![1]]].into();
    let wav2_block2 = WavBlock::from(0.125f32);
    let bub2_field_2: BubbleField = vec![vec![vec![5], vec![5]], vec![vec![5], vec![5]], vec![vec![5], vec![5]], vec![vec![5], vec![5]]].into();
    let bub2_block_1 = BubbleBlock::from_wav_block_and_bub_field(wav2_block1, bub2_field_1);
    let bub2_block_2 = BubbleBlock::from_wav_block_and_bub_field(wav2_block2, bub2_field_2);

    // Floaout block
    let oao_block_1 = FloaoutBlock::from(vec![bub1_block_1, bub2_block_1]);
    let oao_block_2 = FloaoutBlock::from(vec![bub1_block_2, bub2_block_2]);
    // Floaout blocks
    let oao_block_vec = vec![oao_block_1, oao_block_2];
    let write_oao_blocks = FloaoutBlocks::from(oao_block_vec.into_boxed_slice());
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(&write_oao)?;
    writer.write_bubs_details(&write_bubs_in_oao)?;
    writer.write_blocks(&write_oao, write_oao_blocks.clone())?;

    // Finish writing.
    drop(writer);

    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_oao: Floaout = reader.read_details()?;
    let read_bubs_in_oao: BubblesInFloaout = reader.read_bubs_details(&read_oao)?;
    let read_oao_blocks: FloaoutBlocks = reader.read_blocks(&read_oao)?;

    assert_eq!(read_oao, write_oao);
    assert_eq!(read_bubs_in_oao, write_bubs_in_oao);
    assert_eq!(read_oao_blocks, write_oao_blocks);

    remove_file(file)?;

    Ok(())
}