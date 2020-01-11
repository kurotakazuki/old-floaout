use some::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let blow = Blower::from_file("blower.blow")?;
    blow.info_print();
    let positions = vec![(0, 0, 0), (1, 0, 0)];
    blow.pop(positions)?;

    Ok(())
}