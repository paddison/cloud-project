use std::f64::consts::PI;
use std::io::{Write, Seek};
use sine_generator::WavSpec;
use sine_generator::writer::WavWriter;

// pub struct WavSpec {
//     number_of_channels: u16,
//     sample_rate: u32,
//     bits_per_sample: u16,
// }

struct SineWavSpec {
    wav_spec: WavSpec,
    frequencies: Vec<u16>,
    duration: u16,
    volume: f64,
}

impl SineWavSpec {
    pub fn new(wav_spec: WavSpec, frequencies: Vec<u16>, duration: u16, volume: f64) -> Option<Self> {
       //other/different conditions?
        if frequencies.len() < 1 || duration < 1 {
            None
        } else {
            Some(SineWavSpec { wav_spec, frequencies, duration, volume })
        }
    }
}

fn write_wave<W>(sine_spec: SineWavSpec, mut wav_writer: WavWriter<W>) 
where W: Seek + Write
{
    //what to do with volume?
    //aws lambda integration?
    let all_freqs = write_all_frequencies(&sine_spec);

    if sine_spec.wav_spec.bits_per_sample == 16 {
        for freq in all_freqs.iter() {
            let _ = wav_writer.write_sample((freq * 1. / sine_spec.frequencies.len() as f64) as i16);
        } 
    } else {
    
        for freq in all_freqs.iter() {
            println!("{}", (freq * 1. / sine_spec.frequencies.len() as f64) as u8);
            //println!("{}", (freq * 1. / all_freqs.len() as f64) );
            let _ = wav_writer.write_sample((freq * 1. / sine_spec.frequencies.len() as f64) as u8);
        } 
    }
}

fn write_all_frequencies(sine_spec: &SineWavSpec) -> Vec<f64> {
    let mut all_freqs_u8 : Vec<Vec<u8>> = vec![];
    let mut all_freqs_u16 : Vec<Vec<u16>> = vec![]; 
    let mut data = vec![];
    let x_values = sine_spec.wav_spec.sample_rate * sine_spec.duration as u32;
    // calculate each datapoint indivdually
    // add all together 
    // normalize
    // scale to bits per sample
    // call write
    for i in 0..x_values {
        let mut data_point = 0.;
        for freq in &sine_spec.frequencies {
            data_point  += calculate_frequency(i, *freq, sine_spec.wav_spec.sample_rate)
        }
        data_point /= sine_spec.frequencies.len() as f64;
        // scale to bits_per_sample (you can do it!!!!) 
        // 8 bit are unsigned, 16 bit signed
        // signed means can be negative
        // negative means it can be less than 0
        
        // 0 is a number (see https://en.wikipedia.org/wiki/0 for reference)
        data_point = match sine_spec.wav_spec.bits_per_sample {
            8 => (data_point * 128.) as i16 + 127,
            16 => (data_point * i16::MAX) as i16 + 127,
            _ => 0.,
        }

        // return data point
    }

    if sine_spec.wav_spec.bits_per_sample == 16 {
        for freq in sine_spec.frequencies.iter() {
            all_freqs_u16.push(write_frequency_16(*freq, sine_spec.duration, sine_spec.wav_spec.sample_rate));
        }
        for i in 0..(sine_spec.duration as u32 * sine_spec.wav_spec.sample_rate) {
            let mut freq_sum : f64 = 0.;
            for freq in all_freqs_u16.iter() { freq_sum += freq[i as usize] as f64 }
            data.push(freq_sum);
            
        }
    } else {
        for freq in sine_spec.frequencies.iter() {
            all_freqs_u8.push(write_frequency_8(*freq, sine_spec.duration, sine_spec.wav_spec.sample_rate));
        }
        for i in 0..(sine_spec.duration as u32 * sine_spec.wav_spec.sample_rate) {
            let mut freq_sum : f64 = 0.;
            for freq in all_freqs_u8.iter() { freq_sum += freq[i as usize] as f64 }
            data.push(freq_sum);
            
        }
    }
    
    data
}

#[inline(always)]
fn calculate_frequency(i: u32, freq: u16, sample_rate: u32) -> f64{
    let x = (i as f64 / sample_rate as f64) * freq as f64 * 2. * PI;
    x.sin()
}

fn write_frequency_8(freq: u16, duration: u16, sample_rate: u32) -> Vec<u8> {
    let mut data = vec![]; // data size will be sample rate * seconds
    for i in 0..(sample_rate * duration as u32) {
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

fn write_frequency_16(freq: u16, duration: u16, sample_rate: u32) -> Vec<u16> {
    let mut data = vec![]; // data size will be sample rate * seconds
    for i in 0..(sample_rate * duration as u32) {
        let x = (i as f64 / sample_rate as f64) * freq as f64 * 2. * PI;
        data.push(match (x.sin() * 32768.) as i16 + 32767 {
            y if { y >= 0 } => y as u16,
            y => {
                println!("value was less than 0 at i = {}, y = {}", i, y);
                0
            },  
        });
    }
    data
}


fn main() {

    let spec = WavSpec::new(1, 44100, 8).expect("Invalid specification");
    //let sinespec = ...
    let mut writer = WavWriter::new_with_spec(spec, "test_8.wav").expect("Error creating file");
    //write_wav(sinespec)...
    let freqs: Vec<u16> = vec![440, 440 * 5 / 4]; // ein vektor aus frequenzen
    let sine_spec = SineWavSpec::new(spec, freqs.to_vec(), 20, 0.).expect("invalid specification");

    write_wave(sine_spec, writer);
}

// for data value in 0..sample_rate * duration
// for frequency in frequencies
// sum 
// sum each frequency

