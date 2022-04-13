use std::fs::File;
use std::io::prelude::*;
use std::f64::consts::PI;
// use wav_writer::*;

fn main() {
    let header: [u8; 44] = [0x52, 0x49, 0x46, 0x46,     // ChunkID ("RIFF")
                            0x60, 0x1F, 0x10, 0x00,     // ChunkSize 8032
                            0x57, 0x41, 0x56, 0x45,     // Format ("WAVE")
                            0x66, 0x6D, 0x74, 0x20,     // Subchunk1ID ("fmt ")
                            0x10, 0x00, 0x00, 0x00,     // Subchunk1Size (16 = PCM)
                            0x01, 0x00, 0x01, 0x00,     // AudioFormat (PCM = 1) + NumChannels (1 = Mono )
                            0x40, 0x1F, 0x00, 0x00,     // SampleRate (8000)
                            0x40, 0x1F, 0x00, 0x00,     // ByteRate (SampleRate * NumChannels * BitsPerSample / 8)
                            0x01, 0x00, 0x08, 0x00,     // BlockAlign (NumChannles * BitsPerSample / 8) + BitsPerSample (8)
                            0x64, 0x61, 0x74, 0x61,     // Subchunk2ID ("data")
                            0x40, 0x1F, 0x00, 0x00];    // Subchunk2Size (NumSamples * NumChannles * BitsPerSample / 8 = 8000)

    let lower = write_frequency(220);
    let higher = write_frequency(220 * 13 / 12);
    let mut data = [0; 8000];
    for (i, (prim, fifth)) in lower.iter().zip(higher.iter()).enumerate() {
        data[i] = ((*prim as u16 + *fifth as u16) as f64 * 0.5) as u8; 
    }

    write_to_file(&header, &data);
}

fn write_frequency(freq: u32) -> [u8; 8000] {
    let mut data = [0 as u8; 8000]; // data size will be sample rate * seconds
    for i in 0..8000 {
        let x = (i as f64 / 8000 as f64) * freq as f64 * 2. * PI;
        data[i] = match (x.sin() * 128.) as i16 + 127 {
            y if { y >= 0 } => y as u8,
            y => {
                println!("value was less than 0 at i = {}, y = {}", i, y);
                0
            },  
        };
    }
    data
}

fn write_to_file(header: &[u8], data: &[u8]) {
    let mut pos = 0;
    let mut buffer = match File::create("wav_data.wav") {
        Ok(f) => f,
        Err(e) => panic!("Error creating file: {}", e),
    }; 

    while pos < header.len() {
        match buffer.write(&header[pos..]) {
            Ok(bytes_written) => pos += bytes_written,
            Err(e) => panic!("Error writing to file: {}", e),
        }
    }

    pos = 0;
    while pos < data.len() {
        match buffer.write(&data[pos..]) {
            Ok(bytes_written) => pos += bytes_written,
            Err(e) => panic!("Error writing to file: {}", e),
        }
    }
}

#[test]
fn write_le_u24() {
    let val: u32 = 12345;
    let mut buf = [0u8, 3];
    println!("{}", ((val >> 0) & 0xff) as u8);
    println!("{}", ((val >> 0)) as u8);
    println!("{}", ((val >> 8) & 0xff) as u8);
    println!("{}", ((val >> 8)) as u8);
    println!("{}", ((val >> 16) & 0xff) as u8);
    println!("{}", ((val >> 16)) as u8);
}
