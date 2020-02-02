use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::blow::{Blower, BubbleInBlower, BubblesInBlower};
use floaout::io::read::{ReadFmt, ReadFmtFor};
use floaout::io::write::WriteFmt;

#[test]
fn blow_details_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test.blow";
    // Details of Blower that is going to be written.
    let write_blow = Blower {
        version: 0,
        length: 1,
        width: 1,
        height: 1,
        bubbles: 2,
        blocks: 8,
        sampling_rate: 44100,
        bits_per_sample: 32
    };
    // Details of BubblesInBlower
    let write_bub1_in_blow = BubbleInBlower {
        name_size: 3,
        name: "た".into(),
        times: 2,
        ranges: vec!{(0, 2), (4, 6)}
    };
    let write_bub2_in_blow = BubbleInBlower {
        name_size: 0,
        name: "".into(),
        times: 0,
        ranges: vec!{}
    };
    let write_bubs_in_blow: BubblesInBlower = vec!(write_bub1_in_blow, write_bub2_in_blow).into();
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(write_blow)?;
    writer.write_details(write_bubs_in_blow.clone())?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_blow: Blower = reader.read_details()?;
    let read_bubs_in_blow: BubblesInBlower = reader.read_details_for(read_blow.bubbles as usize)?;

    assert_eq!(read_blow, write_blow);
    assert_eq!(read_bubs_in_blow, write_bubs_in_blow);

    remove_file(file)?;

    Ok(())
}