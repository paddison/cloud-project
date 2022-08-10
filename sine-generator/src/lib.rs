use std::io;
use wav_writer::WriteExtension;

#[cfg(feature = "writers")]
pub mod wav_writer;
#[cfg(feature = "writers")]
pub mod frequency_writer;
#[cfg(feature = "data")]
pub mod data_formats;


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