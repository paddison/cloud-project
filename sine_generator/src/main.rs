use std::f64::consts::PI;
use sine_generator::WavSpec;
use sine_generator::writer::WavWriter;

fn main() {

    // testing out the writer
    let time = 60;
    let root = 220;
    let prim = write_frequency(root, 44100, time);
    let third = write_frequency(root * 5 / 4, 44100, time);
    let fifth = write_frequency(root * 3 / 2, 44100, time);
    // let fifth
    let spec = WavSpec::new(1, 44100, 8).expect("Invalid specification");
    let mut writer = WavWriter::new_with_spec(spec, "test_8.wav").expect("Error creating file");
    for ((prim, third), fifth) in prim.iter().zip(third.iter()).zip(fifth.iter()) {
        let _ = writer.write_sample(((*prim as u16 + *third as u16 + *fifth as u16) as f64 * 1. / 3.) as u8); 
    }
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
