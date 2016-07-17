extern crate byteorder;
use std::io::Write;
use std::io::Result;
use byteorder::{BigEndian, WriteBytesExt};

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

struct WavHeader {
    chunk_id : [u8; 4],
    chunk_size: u32,
    format : [u8; 4],
    fmt : Format,
    data : Data,
}

impl WavHeader {
    fn new(sample_rate: u32, bytes_per_sample: usize, data: &[u8]) -> WavHeader {
        WavHeader {
            chunk_id: [b'R', b'I', b'F', b'X'],
            chunk_size: 32 + data.len() as u32,
            format: [b'W', b'A', b'V', b'E'],
            fmt: Format::new(sample_rate, bytes_per_sample),
            data: Data::new(data),
        }
    }

    fn write(&self, writer: &mut Write) -> Result<usize> {
        try!(writer.write(&self.chunk_id));
        try!(writer.write_u32::<BigEndian>(self.chunk_size));
        try!(writer.write(&self.format));
        try!(self.fmt.write(writer));
        try!(self.data.write(writer));
        Ok(self.chunk_size as usize + 8)
    }
}

struct Format {
    chunk_id : [u8; 4],
    chunk_size : u32,
    audio_format : u16,
    channels : u16,
    sample_rate : u32,
    byte_rate : u32,
    block_align : u16,
    bits_per_sample : u16,
}

impl Format {
    fn new(sample_rate: u32, bytes_per_sample: usize) -> Format {
        Format {
            chunk_id: [b'f', b'm', b't', b' '],
            chunk_size: 16,
            audio_format: 1,
            channels: 1,
            sample_rate: sample_rate,
            byte_rate: bytes_per_sample as u32 * sample_rate,
            block_align: bytes_per_sample as u16,
            bits_per_sample: bytes_per_sample as u16 * 8,
        }
    }

    fn write(&self, writer: &mut Write) -> Result<usize> {
        try!(writer.write(&self.chunk_id));
        try!(writer.write_u32::<BigEndian>(self.chunk_size));
        try!(writer.write_u16::<BigEndian>(self.audio_format));
        try!(writer.write_u16::<BigEndian>(self.channels));
        try!(writer.write_u32::<BigEndian>(self.sample_rate));
        try!(writer.write_u32::<BigEndian>(self.byte_rate));
        try!(writer.write_u16::<BigEndian>(self.block_align));
        try!(writer.write_u16::<BigEndian>(self.bits_per_sample));
        Ok(self.chunk_size as usize + 8)
    }

}

struct Data {
    chunk_id : [u8; 4],
    chunk_size : u32,
    data : Vec<u8>,
}

impl Data {
    fn new(data: &[u8]) -> Data {
        Data {
            chunk_id: [b'd', b'a', b't', b'a'],
            chunk_size: data.len() as u32,
            data: data.to_vec(),
        }
    }

    fn write(&self, writer: &mut Write) -> Result<usize> {
        try!(writer.write(&self.chunk_id));
        try!(writer.write_u32::<BigEndian>(self.chunk_size));
        try!(writer.write(self.data.as_slice()));
        Ok(self.chunk_size as usize + 8)
    }
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
    let wave_file = WavHeader::new(sample_rate as u32, std::mem::size_of::<u8>(), &out);

    match wave_file.write(&mut sout) {
        Ok(_) => {},
        Err(e) => {
            println!("Received error [{}] writing output", e);
            std::process::exit(3);
        }
    };
}
