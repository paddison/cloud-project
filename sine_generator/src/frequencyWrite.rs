use std::f64::consts::PI;
use sine_generator::WavSpec;
use sine_generator::writer::WavWriter;

// pub struct WavSpec {
//     number_of_channels: u16,
//     sample_rate: u32,
//     bits_per_sample: u16,
// }

struct SineWavSpec {
    wav_spec: &WavSpec,
    frequencies: Vec<u16>,
    duration: u16,
    volume: f64,
}

impl SineWavSpec {
    pub fn new(wav_spec: &WavSpec, frequencies: Vec<u16>, duration: u16, volume: f64) -> Option<Self> {
       //other/different conditions?
        if frequencies.len() < 1 || duration < 1 {
            None
        } else {
            Some(SineWavSpec { wav_spec, frequencies, duration, volume })
        }
    }
}

fn write_wave(sine_spec: SineWavSpec, wav_writer: WavWriter<W>) {
    //what to do with volume?
    //aws lambda integration?
    let all_freqs = write_all_frequencies(sine_spec);

    if sine_spec.wav_spec.bits_per_sample == 16 {
        for freq in all_freqs {
            let _ = wav_writer.write_sample((freq * 1. / all_freqs.len()) as u16);
        } 
    } else {
        for freq in all_freqs {
            let _ = wav_writer.write_sample((freq * 1. / all_freqs.len()) as u8);
        } 
    }
}

fn write_all_frequencies(sine_spec: SineWavSpec) -> Vec<f64> {
    let mut all_freqs= vec![]; 
    let mut data = vec![];

    if sine_spec.wav_spec.bits_per_sample == 16 {
        for freq in sine_spec.frequencies {
            all_freqs.push(write_frequency_16(*freq, sine_spec.duration, sine_spec.wav_spec.sample_rate));
        }
    } else {
        for freq in sine_spec.frequencies {
            all_freqs.push(write_frequency_8(*freq, sine_spec.duration, sine_spec.wav_spec.sample_rate));
        }
    }
    
    for i in 0..(sine_spec.duration * sine_spec.wav_spec.sample_rate) {
        let mut freq_sum : f64 = 0;
        for freq in all_freqs { freq_sum += freq[i] }
        data.push(freq_sum);
        
    }
    data
}

fn write_frequency_8(freq: u16, duration: u16, sample_rate: u32) -> Vec<u8> {
    let mut data = vec![]; // data size will be sample rate * seconds
    for i in 0..(sample_rate * duration) {
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
    for i in 0..(sample_rate * duration) {
        let x = (i as f64 / sample_rate as f64) * freq as f64 * 2. * PI;
        data.push(match (x.sin() * 32768.) as i16 + 32.767 {
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
    let freqs = [440, 440 * 5 / 4];
    let sine_spec = SineWavSpec::new(spec, freqs, 20, 0);

    write_wav(sine_spec, writer);
}
