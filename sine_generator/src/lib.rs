// pub mod writer;
/// A struct containing metadata about the Wave file that will be created.
pub struct WavSpec {
    number_of_channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
}

impl WavSpec {
    pub fn new(number_of_channels: u16, sample_rate: u32, bits_per_sample: u16) -> Self {
        WavSpec { number_of_channels, sample_rate, bits_per_sample }
    }
}