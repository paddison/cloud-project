use std::io::{Seek, Write, self, BufWriter, SeekFrom };
use std::fs::File;

use crate::{ data_formats::WavSpec, Sample };

/// Provides functionality in order to write numbers in lesser endian 
/// which is required for the data fields
pub trait WriteExtension: Write
{
    fn write_u8(&mut self, n: u8) -> io::Result<()>;
    fn write_le_u16(&mut self, n: u16) -> io::Result<()>;
    fn write_le_u32(&mut self, n: u32) -> io::Result<()>;
}

impl<W> WriteExtension for W
where W: Write
{
    #[inline(always)]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
       self.write_all(&[n])
    }

    #[inline(always)]
    fn write_le_u16(&mut self, n: u16) -> io::Result<()> {
        let mut buffer = [0; 2];
        buffer[0] = (n & 255) as u8;
        buffer[1] = (n >> 8) as u8; 
        self.write_all(&buffer)
    }

    #[inline(always)]
    fn write_le_u32(&mut self, n: u32) -> io::Result<()> {
        let mut buffer = [0; 4];
        buffer[0] = (n & 255) as u8;
        buffer[1] = ((n >> 8) & 255) as u8;
        buffer[2] = ((n >> 16) & 255) as u8;
        buffer[3] = (n >> 24) as u8;
        self.write_all(&buffer)
    }
}

#[test]
fn write_u8() {
    let mut buffer = vec![];
    {
        let mut writer = BufWriter::new(&mut buffer);
        let _ = writer.write_u8(24).expect("Error occured while testing write_u8");
    }
    assert_eq!(buffer[0], 24);
}

#[test]
fn write_le_u16() {
    let mut buffer = vec![];
    {
        let mut writer = BufWriter::new(&mut buffer);
        let _ = writer.write_le_u16(0b1110_1101_0001_0001).expect("Error occured while testing write_le_u16");
    }
    assert_eq!(buffer[0], 0b0001_0001);
    assert_eq!(buffer[1], 0b1110_1101);
}
#[test]
fn write_le_u16_as_i16() {
    let mut buffer = vec![];
    {
        let mut writer = BufWriter::new(&mut buffer);
        let n: i16 = -4847;
        let _ = writer.write_le_u16(n as u16).expect("Error occured while testing write_le_u16");
    }
    assert_eq!(buffer[0], 0b0001_0001);
    assert_eq!(buffer[1], 0b1110_1101);
}

#[test]
fn write_le_u32() {
    let mut buffer = vec![];
    {
        let mut writer = BufWriter::new(&mut buffer);
        let _ = writer.write_le_u32(0b1110_1101_0001_0001_1101_1000_0010_0000).expect("Error occured while testing write_le_u32");
    }
    assert_eq!(buffer[0], 0b0010_0000);
    assert_eq!(buffer[1], 0b1101_1000);
    assert_eq!(buffer[2], 0b0001_0001);
    assert_eq!(buffer[3], 0b1110_1101);
}

/// The `WavWriter` is the main interface used by the application
/// to create a Wave file and write data into it.
pub struct WavWriter<W> 
where W: Write + Seek
{
    writer: ChunkWriter<W>,
}

impl<W> WavWriter<W>
where W: Write + Seek
{
    #[inline(always)]
    pub fn write_sample<S: Sample>(&mut self, value: S) -> io::Result<u32> {
        self.writer.write(value)
    }

    /// An explicit way of flushing the writer.
    /// Returns an `io::Result<()>` which can be used for error checking
    pub fn finalize(&mut self) -> io::Result<()> {
        self.writer.finalize()
    }
}

impl WavWriter<BufWriter<File>> {
    pub fn new_with_spec(spec: WavSpec, file_name: &str) -> Result<WavWriter<BufWriter<File>>, io::Error>  {
        let file = File::create(file_name)?; 
        let writer = BufWriter::new(file);
        Ok(WavWriter { 
            writer: ChunkWriter::initialize_with_spec(spec, writer)?
        })
    }
}

/// The internal writer used by the `WavWriter`, which provides all the functionality of creating a 
/// Wave file and writing all the necessary data, and maintaining the state of the Data Chunk. 
struct ChunkWriter<W> 
where W: Write + Seek
{
    spec: WavSpec,
    writer: W,
    data_state: DataState,
}

impl<W> ChunkWriter<W>
where W: Write + Seek 
{
    /// Initializes a new `ChunkWriter` with a `WavSpec` struct.
    /// Upon initialization, it will immediately write the fields
    /// of the Wave file header, except for file sizes.
    fn initialize_with_spec(spec: WavSpec, writer: W) -> Result<ChunkWriter<W>, io::Error> {
        let mut chunk_writer = Self { spec, writer, data_state: DataState { bytes_written: 0, dirty: true } };
        chunk_writer.write_header()?;
        Ok(chunk_writer)
    }

    /// Writes the Wave header into the buffer.
    fn write_header(&mut self) -> io::Result<()> {
        self.writer.write_all(b"RIFF\0\0\0\0WAVEfmt ")?;
        self.writer.write_le_u32(16)?;  // Subchunk2 Size
        self.writer.write_le_u16(1)?;   // 1 = PCM
        self.writer.write_le_u16(self.spec.number_of_channels)?;
        self.writer.write_le_u32(self.spec.sample_rate)?;
        self.writer.write_le_u32(self.spec.sample_rate * self.spec.number_of_channels as u32 * self.spec.bits_per_sample as u32 / 8)?;
        self.writer.write_le_u16(self.spec.number_of_channels * self.spec.bits_per_sample / 8)?;
        self.writer.write_le_u16(self.spec.bits_per_sample)?;
        self.writer.write_all(b"data\0\0\0\0")?;
        Ok(())
    }

    /// Update the chunk size fields in the header
    /// length is the total amount of sample data written
    fn update_chunk_size(&mut self) -> io::Result<()> {
        let length = self.data_state.bytes_written;
        self.writer.seek(SeekFrom::Start(4))?;      // update ChunkSize field
        self.writer.write_le_u32(length + 36)?;
        self.writer.seek(SeekFrom::Start(40))?;     // update Subchunk2Size field
        self.writer.write_le_u32(length)?;
        self.writer.seek(SeekFrom::End(0))?;
        Ok(())
    }

    /// Writes a sample
    /// For now, stereo just writes the sample two times
    #[inline(always)]
    fn write<S: Sample>(&mut self, value: S) -> io::Result<u32> {
        let byte_rate = self.spec.bits_per_sample as u32 / 8;
        value.write(&mut self.writer)?;
        self.data_state.bytes_written += byte_rate;
        if self.spec.number_of_channels == 2 {
            value.write(&mut self.writer)?;
            self.data_state.bytes_written += byte_rate;
        }
        Ok(byte_rate)
    }

    /// Updates the header and checks if data is of valid length.
    /// Then flushes the writer
    fn flush(&mut self) -> io::Result<()> {
        if !self.data_state.is_valid_length() {
            self.data_state.bytes_written += match self.spec.bits_per_sample {
                8 => self.write(0_u8),
                16 => self.write(0_i16),
                _ => panic!("Only 8 or 16 bit are supported"),
            }?;
        }
        self.data_state.dirty = false;
        self.update_chunk_size()?;
        self.writer.flush()?;
        Ok(())
    }

    /// An explicit way of flushing the writer.
    /// This way, it can be checked if all the data was written correctly,
    /// since it returns a `io::Result<()>`, which is not possible
    /// if it is dropped
    fn finalize(&mut self) -> io::Result<()> {
        self.flush()
    }
}

#[test]
fn write_header() {
    use std::io::Cursor;

    let spec = WavSpec::new(2, 22050, 16).unwrap();
    let writer = BufWriter::new(Cursor::new(vec![]));
    let chunky = ChunkWriter::initialize_with_spec(spec, writer).unwrap();
    let data = chunky.writer.buffer();
    assert_eq!(data[0..4], 0x52_49_46_46_u32.to_be_bytes());        // RIFF
    assert_eq!(data[4..8], 0_u32.to_be_bytes());                    // Chunksize = 0
    assert_eq!(data[8..12], 0x57_41_56_45_u32.to_be_bytes());       // WAVE
    assert_eq!(data[12..16], 0x66_6d_74_20_u32.to_be_bytes());      // fmt 
    assert_eq!(data[16..20], 0x10_00_00_00_u32.to_be_bytes());      // Subchunk1Size = 16
    assert_eq!(data[20..22], 0x01_00_u16.to_be_bytes());            // AudioFormat = 1 (PCM)
    assert_eq!(data[22..24], 0x02_00_u16.to_be_bytes());            // NumChannels = 2
    assert_eq!(data[24..28], 0x22_56_00_00_u32.to_be_bytes());      // SampleRate = 22050
    assert_eq!(data[28..32], 0x88_58_01_00_u32.to_be_bytes());      // ByteRate = 88200
    assert_eq!(data[32..34], 0x04_00_u16.to_be_bytes());            // BlockAlign = 4
    assert_eq!(data[34..36], 0x10_00_u16.to_be_bytes());            // BitsPerSample = 16
    assert_eq!(data[36..40], 0x64_61_74_61_u32.to_be_bytes());      // data
    assert_eq!(data[40..44], 0_u32.to_be_bytes());                  // Subchunk2Size = 0
}

/// Upon dropping the `ChunkWriter`, it is necessary 
/// to update the length fields, and verify if the 
/// data size is valid
impl<W> Drop for ChunkWriter<W> 
where W: Write + Seek
{
    fn drop(&mut self) {
        if self.data_state.dirty {
            let _ = self.flush();
        }
    }
}

/// Contains the state of the data of the file.
/// 
/// `bytes_written`: The amount of data bytes written so far
/// 
/// `dirty`: Boolean indicating if the data state is clean or not
/// The state will be dirty for the duration of the write.
/// When finalizing the write, the written data is checked
/// for an even length (according to the WAVE Specification). 
/// If this is not the case, a last "filler"-byte needs to be written, 
/// before setting it to not dirty.
struct DataState {
    bytes_written: u32,
    dirty: bool,
}

impl DataState {
    /// Verify if bytes written is an even number
    fn is_valid_length(&self) -> bool {
        self.bytes_written % 2 == 0
    }
}