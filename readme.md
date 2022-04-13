# Project for Software Development for Cloud Computing

## Description

A cloud application, in which a user can specify a set of parameters to create a wav file containing sine waves of different frequencies, which then can be downloaded on his/her computer.

The main application can be found in `sine_generator`.

An example application of how to integrate rust with aws using a lambda and a bucket can be found [here](https://github.com/paddison/aws-rust-le-solver).

# Specification

This is a first specification, trying to define the rough design of the application.

## General 

The app should be structured into two main parts. A small hosted website, from which a user can make a request to create a file, and a lambda function (or something similar), which reads the request, and creates the file.

The file itself might be saved in a bucket. The website might ask the bucket for the status of the file-creation
with a specified id. After a file with the id is found, it might be sent to the user to download it.
After the download was completed, the file may be deleted from the bucket

It might look something like this:

```
+----------+                      +-------------------+              +--------+
| WebPage  |                      | Lambda Function   |              | Bucket |
| with     |    Request (JSON)    | Read JSON         |  Store File  |        |
| GUI      |--------------------->| Create File       |------------->|        |
+----------+                      +-------------------+              +--------+
           |      Send status request, and reply                     |     
           |<------------------------------------------------------->|
           |                                                         |
           |      If done, send file                                 |
           |<--------------------------------------------------------|
           |                                                         |
           |      Download finished                                  |
           |-------------------------------------------------------->| Delete file
```

## Wav-File-Creation

The goal shouldn't be to do a complete implementation of the Wave Format, but to provide the necessary functionality to create a correct audio file. Thus, no reading capabilites need to be added, and only PCM (Pulse Code Modulation) format should be implemented.

A user may specify parameters for the Wav file itself: 
- number of channels (1 or 2)
- sample rate (8000, 11025, 22050, 44100)
- bits per sample (8 or 16)

They may also specify parameters for the data of the file:
- Frequency (several possible)
- Length in seconds
- Volume (optional)
  
The data might be transfered in JSON-Format, which might look like this:

```
{
    request: {
        id: string,
        wav_spec: {
            number_of_channels: integer,
            sample_rate: integer,
            bits_per_sample: integer
        },
        wav_data: {
            frequency: Array[int],
            length: float or int,
            volume: float
        }
    }
}
```

## Structs and Functionality

### Writing Data

#### WavSpec

The `wav_spec` json-object should be described by a struct. It might be of the following form:
```
struct WavSpec {
    number_of_channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
}
```

#### WavWriter

The WavWriter should be initialized with the WavSpec struct. It wraps a ChunkWriter, which it delegates the actual writing to. It will be the interface used to write data for the outside.

#### ChunkWriter

Does all the actual writing. It holds information about the WavSpec, a Writer, which will write to a file, and a buffer which holds the data chunk temporarily.
When initializing, it should add the main information of the header into a temporary buffer (e.g. a &[u8; 44] array). After writing all the information of the header, the temporary buffer should be written into the buffer of the writer.
Upon finishing the writing of all the data, it should update the size fields of the header in the its buffer, and then write its whole buffer into the file. This might be implemented through the `Drop` trait, which is a function that gets called when a value goes out of scope.

The ChunkWriter does not know which data it writes into the data chunk, meaning it will have no information on the actual sine wave it will write.

#### Sample

A sample should be implemented as a trait for integer types, in order to write Sample Data in a generic way. E.g. there should be no need to call a write 16 bit function or write 8 bit function, but rather a single function, which passes in the necessary information from the WavSpec.

#### Additional

It might be helpful to create functions that implement writing lesser endian for different integer sizes. They should be implemented for all Types that implement the io::Write trait.

### Generating Sine Wave Functions



## Notes

- Only use PCM (integer sized samples)
- Sample type to abstract away specific integer types. 

## Links

[WAVE PCM soundfile format](http://soundfile.sapp.org/doc/WaveFormat/)

[WAVEFORMTEX](https://docs.microsoft.com/en-us/previous-versions//ms713497(v=vs.85)?redirectedfrom=MSDN)

[hound, a library which implements the Wav spec](https://github.com/ruuda/hound)



