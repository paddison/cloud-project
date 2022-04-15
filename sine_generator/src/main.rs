use std::fs::File;
use std::io::prelude::*;
use std::f64::consts::PI;
use sine_generator::WavSpec;

fn main() {
    let header: [u8; 44] = [0x52, 0x49, 0x46, 0x46,     // ChunkID ("RIFF")
                            0x28, 0x5F, 0xF0, 0x00,     // ChunkSize 8032
                            0x57, 0x41, 0x56, 0x45,     // Format ("WAVE")
                            0x66, 0x6D, 0x74, 0x20,     // Subchunk1ID ("fmt ")
                            0x10, 0x00, 0x00, 0x00,     // Subchunk1Size (16 = PCM)
                            0x01, 0x00, 0x01, 0x00,     // AudioFormat (PCM = 1) + NumChannels (1 = Mono )
                            0x44, 0xAC, 0x00, 0x00,     // SampleRate (8000)
                            0x44, 0xAC, 0x00, 0x00,     // ByteRate (SampleRate * NumChannels * BitsPerSample / 8)
                            0x01, 0x00, 0x08, 0x00,     // BlockAlign (NumChannles * BitsPerSample / 8) + BitsPerSample (8)
                            0x64, 0x61, 0x74, 0x61,     // Subchunk2ID ("data")
                            0x28, 0x5F, 0xF0, 0x00];    // Subchunk2Size (NumSamples * NumChannles * BitsPerSample / 8 = 8000)

    let lower = write_frequency(220, 44100, 60);
    let higher = write_frequency(220 * 5 / 3, 44100, 60);
    let mut data = vec![];
    for (prim, fifth) in lower.iter().zip(higher.iter()) {
        data.push(((*prim as u16 + *fifth as u16) as f64 * 0.5) as u8); 
    }
    println!("{}", data.len());
    write_to_file(&header, &data);
}

fn write_frequency(freq: u32, sample_rate: usize, time: usize) -> Vec<u8> {
    let mut data = vec![]; // data size will be sample rate * seconds
    for i in 0..(sample_rate * time) {
        let x = (i as f64 / sample_rate as f64) * freq as f64 * 2. * PI;
        data.push(match (x.sin() * 128.) as i16 + 127 {
            y if { y >= 0 } => y as u8,
            y => {
                println!("value was less than 0 at i = {}, y = {}", i, y);
                0
            },  
        });
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
