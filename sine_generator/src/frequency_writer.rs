use std::f64::consts::PI;

use crate::wav_writer::WavWriter;
use crate::WavSpec;

pub struct SineWavSpec<'spec> {
    wav_spec: &'spec WavSpec,
    frequencies: Vec<u16>,
    duration: u16,
    volume: f64,
}

impl<'spec> SineWavSpec<'spec> {
    pub fn new(wav_spec: &'spec WavSpec, frequencies: Vec<u16>, duration: u16, volume: f64) -> Option<Self> {
       //other/different conditions?
        if frequencies.len() < 1 || duration < 1 {
            None
        } else {
            Some(SineWavSpec { wav_spec, frequencies, duration, volume })
        }
    }
}

pub fn write_wave<W: std::io::Write + std::io::Seek>(sine_spec: SineWavSpec, mut wav_writer: WavWriter<W>) {
    //what to do with volume?
    //aws lambda integration?
    let n_data_points =  sine_spec.duration as u32 * sine_spec.wav_spec.sample_rate;

    // closures, which calculates the datapoints for different bit sizes
    let f_16 = |x: f64| (x.sin() * 32768. - 0.5) as isize;
    let f_8 = |x: f64| match (x.sin() * 128.) as isize + 127 {
        y if { y >= 0 } => y as isize,
        _ => 0,
    };

    // decide which closure should be used, according to the bit size
    let f = match &sine_spec.wav_spec.bits_per_sample {
        16 => f_16,
        8 => f_8,
        bit_size => panic!("Unsupported bit size: {}", bit_size)
    };

    // calculate each data point and write them out
    for i in 0..n_data_points {
        let mut sample = 0;
        for freq in &sine_spec.frequencies {
            let x = (i as f64 /  sine_spec.wav_spec.sample_rate as f64) * *freq as f64 * 2. * PI;
            sample += f(x);
        }
        let scaled_sample = sine_spec.volume * sample as f64 / sine_spec.frequencies.len() as f64 ;
        if sine_spec.wav_spec.bits_per_sample == 16 {
            let _ = wav_writer.write_sample(scaled_sample as i16);
        } else {
            let _ = wav_writer.write_sample(scaled_sample as u8);
        }
    }
}