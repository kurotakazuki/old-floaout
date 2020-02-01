use std::io::{BufReader, BufWriter};
use std::fs::File;
use floaout::format::bub::Bubble;
use floaout::io::read::ReadExt;
use floaout::io::write::WriteExt;

#[test]
fn bub_details_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test1.bub";
    // Details of Bubble that is going to be written.
    let write_bub = Bubble {
        version: 0,
        length: 1,
        width: 1,
        height: 1,
        red: 255,
        green: 255,
        blue: 255,
        blocks: 8,
        sampling_rate: 44100,
        bits_per_sample: 32,
        name_size: 3,
        name: "た".to_string(),
        overall: vec![vec![vec![0, 1], vec![2, 3]], vec![vec![4, 5], vec![6, 7]]]
    };
    // Clone Bubble because it's not implementing Clone trait.
    let write_bub_clone = write_bub.clone();
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(write_bub_clone)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_bub: Bubble = reader.read_details()?;

    assert_eq!(read_bub, write_bub);

    Ok(())
}

#[test]
fn bub_details_without_name_test() -> Result<(), Box<dyn std::error::Error>> {
    // File name
    let file = "test2.bub";
    // Details of Bubble that is going to be written.
    let write_bub = Bubble {
        version: 0,
        length: 1,
        width: 1,
        height: 1,
        red: 255,
        green: 255,
        blue: 255,
        blocks: 8,
        sampling_rate: 44100,
        bits_per_sample: 32,
        name_size: 0,
        name: "".to_string(),
        overall: vec![vec![vec![0, 1], vec![2, 3]], vec![vec![4, 5], vec![6, 7]]]
    };
    // Clone Bubble because it's not implementing Clone trait.
    let write_bub_clone = write_bub.clone();
    // Writer
    let mut writer = BufWriter::new(File::create(file)?);
    writer.write_details(write_bub_clone)?;
    // Finish writing.
    drop(writer);
    // Reader
    let mut reader = BufReader::new(File::open(file)?);
    let read_bub: Bubble = reader.read_details()?;

    assert_eq!(read_bub, write_bub);

    Ok(())
}