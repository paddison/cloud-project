use serde::{Deserialize, Serialize};

/// A struct containing metadata about the Wave file that will be created.
#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct WavSpec {
    pub number_of_channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}

impl WavSpec {
    pub fn new(number_of_channels: u16, sample_rate: u32, bits_per_sample: u16) -> Option<Self> {
        if number_of_channels != 1 && number_of_channels != 2 || bits_per_sample != 8 && bits_per_sample != 16 {
            None
        } else {
            Some(WavSpec { number_of_channels, sample_rate, bits_per_sample })
        }
    }
}

impl Verifiable for WavSpec {
    fn is_valid(&self) -> bool {
        self.number_of_channels == 1 || self.number_of_channels == 2 || self.bits_per_sample == 8 || self.bits_per_sample == 16
    }
}

#[derive(Deserialize, Serialize)]
pub struct WavData {
    pub frequencies: Vec<u16>,
    pub duration: u16,
    pub volume: f64,
}

impl Verifiable for WavData {
    fn is_valid(&self) -> bool {
        self.duration > 0 && self.duration <= 60 && self.volume >= 0.0 && self.volume <= 1.0
    }
}

pub trait Verifiable {
    fn is_valid(&self) -> bool;
}

#[test]
fn new_is_some() {
    let spec = WavSpec::new(1, 44100, 8);
    assert!(spec.is_some());
    let spec = WavSpec::new(1, 44100, 16);
    assert!(spec.is_some());
    let spec = WavSpec::new(2, 44100, 8);
    assert!(spec.is_some());
    let spec = WavSpec::new(2, 44100, 16);
    assert!(spec.is_some());
}

#[test]
fn new_is_none() {
    let spec = WavSpec::new(0, 44100, 8);
    assert!(spec.is_none());
    let spec = WavSpec::new(1, 44100, 24);
    assert!(spec.is_none());
}