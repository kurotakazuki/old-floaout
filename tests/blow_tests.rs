use std::io::{BufReader, BufWriter};
use std::fs::{File, remove_file};
use floaout::format::blow::Blower;
use floaout::io::read::ReadExt;
use floaout::io::write::WriteExt;

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
        bubbles: 1,
        blocks: 8,
        sampling_rate: 44100,
        bits_per_sample: 32
    };
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(write_blow)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_blow: Blower = reader.read_details()?;

    assert_eq!(read_blow, write_blow);

    remove_file(file)?;

    Ok(())
}