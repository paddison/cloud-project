use std::io;

use writer::WriteExtension;

pub mod writer;
/// A struct containing metadata about the Wave file that will be created.
#[derive(Clone, Copy)]
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

pub trait Sample {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl Sample for u8 {
    #[inline(always)]
    fn write<W: std::io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(*self)
    }
}

impl Sample for i16 {
    #[inline(always)]
    fn write<W: std::io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_le_u16(*self as u16)
    }
}