fn main() -> Result<(), Box<dyn std::error::Error>> {

    Wav::wav_format("mono.wav")?;
    
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let wav = Wav::from_file("format_mono.wav")?;
    wav.info_print();
    wav.make_file()?;
    let blow = Blower::from_file("blower.blow")?;
    blow.info_print();
    blow.make_file()?;
    let bub = Bubble::from_file("bubble.bub")?;
    bub.info_print();
    bub.make_file()?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Blower {
        bubbles: 3,
        length: 2,
        width: 2,
        height: 1,
        samples: 88200,
        sampling_rate: 44100,
        bits_per_sample: 32,
        title_size: 6,
        title: String::from("blower"),
        artist_size: 7,
        artist: String::from("unknown"),
        bubbles_name_size: vec![5, 5, 5],
        bubbles_name: vec![String::from("test0"), String::from("test1"), String::from("test2")],
        bubbles_time_size: vec![16, 16, 16],
        bubbles_time: vec![(0, 88200), (0, 88200), (0, 88200)],
        format_code: 3,
        .. Default::default()
    };

    let b = a.to_wav_info();

    b.info_print();

    let c = b.with_channels(2);

    c.info_print();

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Wav::from_wav_file("mono.wav")?;
    a.info_print();

    let b = Bubble::from_wav_file("format_mono.wav")?;
    b.info_print();
    
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Blower {
        bubbles: 3,
        length: 2,
        width: 2,
        height: 1,
        samples: 88200,
        sampling_rate: 44100,
        bits_per_sample: 32,
        title_size: 6,
        title: String::from("blower"),
        artist_size: 7,
        artist: String::from("unknown"),
        bubbles_name_size: vec![5, 5, 5],
        bubbles_name: vec![String::from("test0"), String::from("test1"), String::from("test2")],
        bubbles_time_size: vec![16, 16, 16],
        bubbles_time: vec![(0, 88200), (0, 88200), (0, 88200)],
        format_code: 3,
        .. Default::default()
    };

    a.info_print();

    a.make_file()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Bubble {
        length: 2,
        width: 2,
        height: 1,
        red: 24,
        samples: 88200,
        sampling_rate: 44100,
        bits_per_sample: 32,
        name_size: 6,
        name: String::from("bubble"),
        overall_size: 16,
        overall: vec![vec![vec![0.5], vec![0.0]], vec![vec![1.0], vec![0.5]]],
        // overall: vec![0.5, 1.0, 0.0, 0.5],
        format_code: 3,
        .. Default::default()
    };

    a.bubble_make("format_mono.wav")?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    Bubble::bubble_to_wav("bubble.bub", 1, 0, 0)?;
    
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    Bubble::bubble_in_wav("new.wav", "bubble.bub", 1, 0, 0, (0, 88200))?;

    Ok(())
}


use std::fs::File;
use std::io::BufWriter;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Blower {
        bubbles: 3,
        length: 2,
        width: 2,
        height: 1,
        samples: 88200,
        sampling_rate: 44100,
        bits_per_sample: 32,
        title_size: 6,
        title: String::from("blow00"),
        artist_size: 7,
        artist: String::from("unknown"),
        bubbles_name_size: vec![5, 5, 5],
        bubbles_name: vec![String::from("test0"), String::from("test1"), String::from("test2")],
        bubbles_time_size: vec![16, 16, 16],
        bubbles_time: vec![(0, 88200), (0, 88200), (0, 88200)],
        format_code: 3,
        .. Default::default()
    };

    let b = a.to_wav_info();

    b.info_print();


    let new_wav_info = b.with_channels(2);
    new_wav_info.info_print();
    // Make new Wav File
    // Writer
    let new_wav_file = format!("{}.wav", a.title);
    let mut writer = BufWriter::new(File::create(new_wav_file)?);
    let mut writer = new_wav_info.head_writer(writer)?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let a = Blower {
        bubbles: 3,
        length: 2,
        width: 2,
        height: 1,
        samples: 88200,
        sampling_rate: 44100,
        bits_per_sample: 32,
        title_size: 6,
        title: String::from("blow00"),
        artist_size: 7,
        artist: String::from("unknown"),
        bubbles_name_size: vec![5, 5, 5],
        bubbles_name: vec![String::from("test0"), String::from("test1"), String::from("test2")],
        bubbles_time_size: vec![16, 32, 16],
        bubbles_time: vec![(0, 88200), (0, 88200), (0, 88200), (0, 88200)],
        format_code: 3,
        .. Default::default()
    };

    a.info_print();

    let c = a.clone();

    let b = BubblesInBlower::from_blower_info(a);

    println!("{:?}", b);

    c.info_print();

    Ok(())
}