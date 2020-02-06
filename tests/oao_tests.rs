use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::oao::{BubbleInFloaout, BubblesInFloaout, Floaout};
use floaout::io::read::{ReadFmt, ReadBubsIn};
use floaout::io::write::{WriteFmt, WriteBubsIn};

#[test]
fn oao_details_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test.oao";
    // Details of Floaout that is going to be written.
    let write_oao = Floaout {
        version: 0,
        song_id: 0,
        bub_field_size: (1u8, 1u8, 1u8).into(),
        bubbles: 2,
        blocks: 8,
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
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(&write_oao)?;
    writer.write_bubs_details(&write_bubs_in_oao)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_oao: Floaout = reader.read_details()?;
    let read_bubs_in_oao: BubblesInFloaout = reader.read_bubs_details_for(read_oao.bubbles as usize)?;

    assert_eq!(read_oao, write_oao);
    assert_eq!(read_bubs_in_oao, write_bubs_in_oao);

    remove_file(file)?;

    Ok(())
}