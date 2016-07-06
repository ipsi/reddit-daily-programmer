use std::io::Write;

fn frequencies(note: char) -> f32 {
    match note {
        'A' => return 440.00,
        'B' => return 493.88,
        'C' => return 261.63,
        'D' => return 293.66,
        'E' => return 329.63,
        'F' => return 349.23,
        'G' => return 392.00,
        _ => return 0.00,
    }
}

struct wav_header {
    chunk_id : str = "RIFF",
    chunk_size: u32,
    format : str = "WAVE",
    subchunk_id : str = "fmt ",
    subchunk_size : u32 = 16,
    audio_format : u16 = 1,
    channels : u16 = 1,
    sample_rate : u32,
    byte_rate : u32,
    block_align : u16,
    bits_per_sample : u16,
    data_id : str = "data",
    data_size : u32,
    data : [u8],
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let sample_rate = match args[1].parse::<u16>() {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to convert sample_rate [{}] to an int with message [{}]!",  args[1], e);
            std::process::exit(1);
        }
    };
    let duration = match args[2].parse::<u16>() {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to convert duration [{}] to int with message [{}]!", args[2], e);
            std::process::exit(2);
        }
    };
    let notes = &args[3];

    //println!("Sample Rate: {}, Duration: {}, Notes: {}", sample_rate, duration, notes);

    let tmp = duration as f64 / 1000.0;
    //println!("duration as f64 / 1000.0 = {}", tmp);
    let samples_f64 = sample_rate as f64 * tmp;
    //println!("sample_rate as f64 * tmp = {}", samples_f64);
    let samples = samples_f64 as u16;
    let mut out: Vec<u8> = Vec::new();
    for note in notes.chars() {
        let frequency = frequencies(note);
        let wavelength = sample_rate as f32 / frequency;
        for t in 0..samples {
            //println!("Producing sample for t={}, frequency={}, wavelength={}", t, frequency, wavelength);
            let twopi = 2.0 * std::f32::consts::PI;
            //println!("twopi {}", twopi);
            let twopit = twopi * (t as f32);
            //println!("twopit {}", twopit);
            let twopitwav = twopit / wavelength;
            //println!("twopitwav {}", twopitwav);
            let twopitwavsin = twopitwav.sin();
            //println!("twopitwavsin {}", twopitwavsin);
            out.push(((128.0 * twopitwavsin) as i8) as u8);
        }
    }

    let mut sout = std::io::stdout();
    match sout.write_all(&out) {
        Ok(_) => {},
        Err(e) => {
            println!("Received error [{}] writing output", e);
            std::process::exit(3);
        }
    };
}
