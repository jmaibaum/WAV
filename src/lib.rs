//! This is a crate for reading in and writing out wave files. It supports
//! uncompressed PCM bit depths of 8, 16, 24 bits, and 32bit IEEE Float formats,
//! both with any number of channels, Unfortunately other types of data format
//! (e.g. compressed WAVE files) are not supported. There is also no support for
//! any metadata chunks or any chunks other than the "fmt " and "data" chunks.
//!
//! ## Example
//!
//! ```rust
//! # fn main() -> std::io::Result<()> {
//! use std::fs::File;
//! use std::path::Path;
//!
//! let mut inp_file = File::open(Path::new("data/sine.wav"))?;
//! let (header, data) = wav::read(&mut inp_file)?;
//!
//! let mut out_file = File::create(Path::new("data/output.wav"))?;
//! wav::write(header, &data, &mut out_file)?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::{
    convert::TryFrom,
    io::{self, Read, Write},
};

pub mod header;
pub use header::{Header, WAV_FORMAT_IEEE_FLOAT, WAV_FORMAT_PCM};

pub mod bit_depth;
pub use bit_depth::BitDepth;

mod tuple_iterator;
use tuple_iterator::{PairIter, QuadrupletIter, TripletIter};

/// Reads in the given `reader` and attempts to extract the audio data and
/// header from it.
///
/// ## Errors
///
/// This function fails under the following circumstances:
/// * Any error occurring from the `reader` parameter during reading.
/// * The data isn't RIFF data.
/// * The wave header specifies a compressed data format.
/// * The wave header specifies an unsupported bit-depth.
/// * The wave data is malformed, or otherwise couldn't be parsed into samples.
#[allow(clippy::similar_names)]
pub fn read<R>(reader: &mut R) -> io::Result<(Header, BitDepth)>
where
    R: Read + io::Seek,
{
    let header = read_header(reader)?;
    Ok((header, read_data(reader, &header)?))
}

/// Writes the given wav data to the given `writer`.
///
/// ## Notes
///
/// Although `track` is a borrowed value, its contents will be formatted into an
/// owned `Vec<u8>` so that it can be written to the `writer` through
/// [`riff::ChunkContents::write`].
///
/// ## Errors
///
/// This function fails under the following circumstances:
/// * Any error occurring from the `writer` parameter during writing.
/// * The given `BitDepth` is `BitDepth::Empty`.
pub fn write<W>(header: Header, track: &BitDepth, writer: &mut W) -> std::io::Result<()>
where
    W: Write + io::Seek,
{
    let w_id = riff::ChunkId { value: [b'W', b'A', b'V', b'E'] };

    let h_id = riff::ChunkId { value: [b'f', b'm', b't', b' '] };
    let h_vec: [u8; 16] = header.into();
    let h_dat = riff::ChunkContents::Data(h_id, Vec::from(&h_vec[0..16]));

    let d_id = riff::ChunkId { value: [b'd', b'a', b't', b'a'] };
    let d_vec = match track {
        BitDepth::Eight(v) => v.clone(),
        BitDepth::Sixteen(v) => v
            .iter()
            .flat_map(|s| {
                let v = s.to_le_bytes();
                PairIter::new((v[0], v[1]))
            })
            .collect::<Vec<_>>(),
        BitDepth::TwentyFour(v) => v
            .iter()
            .flat_map(|s| {
                let v = s.to_le_bytes().split_at(1).1.to_owned();
                TripletIter::new((v[0], v[1], v[2]))
            })
            .collect::<Vec<_>>(),
        BitDepth::ThirtyTwoFloat(v) => v
            .iter()
            .flat_map(|s| {
                let v = s.to_le_bytes().to_owned();
                QuadrupletIter::new((v[0], v[1], v[2], v[3]))
            })
            .collect::<Vec<_>>(),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Empty audio data given",
            ))
        }
    };
    let d_dat = riff::ChunkContents::Data(d_id, d_vec);

    let r = riff::ChunkContents::Children(riff::RIFF_ID.clone(), w_id, vec![h_dat, d_dat]);

    r.write(writer)?;

    Ok(())
}

#[allow(clippy::similar_names)]
fn read_header<R>(reader: &mut R) -> io::Result<Header>
where
    R: Read + io::Seek,
{
    let wav = verify_wav_file(reader)?;

    for c in wav.iter(reader) {
        if c.id().as_str() == "fmt " {
            // Read header contents
            let header_bytes = c.read_contents(reader)?;
            let header = Header::try_from(header_bytes.as_slice())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Return error if not using PCM
            match header.audio_format {
                WAV_FORMAT_PCM | WAV_FORMAT_IEEE_FLOAT => return Ok(header),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unsupported data format, data is not in uncompressed PCM format, aborting",
                    ))
                }
            };
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "RIFF data is missing the \"fmt \" chunk, aborting",
    ))
}

#[allow(clippy::similar_names)]
fn read_data<R>(reader: &mut R, header: &Header) -> io::Result<BitDepth>
where
    R: Read + io::Seek,
{
    let wav = verify_wav_file(reader)?;

    for c in wav.iter(reader) {
        if c.id().as_str() == "data" {
            // Read data contents
            let data_bytes = c.read_contents(reader)?;

            let wav_data = match header.audio_format {
                WAV_FORMAT_PCM => match header.bits_per_sample {
                    8 => Ok(BitDepth::Eight(data_bytes)),
                    16 => Ok(BitDepth::Sixteen({
                        let mut tmpv = Vec::with_capacity(data_bytes.len() / 2);
                        tmpv.extend(
                            data_bytes
                                .chunks_exact(2)
                                .map(|i| i16::from_le_bytes([i[0], i[1]])),
                        );
                        tmpv
                    })),
                    24 => Ok(BitDepth::TwentyFour({
                        let mut tmpv = Vec::with_capacity(data_bytes.len() / 3);
                        tmpv.extend(
                            data_bytes
                                .chunks_exact(3)
                                .map(|i| i32::from_le_bytes([0, i[0], i[1], i[2]])),
                        );
                        tmpv
                    })),
                    _ => Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unsupported PCM bit depth",
                    )),
                },
                WAV_FORMAT_IEEE_FLOAT => match header.bits_per_sample {
                    32 => Ok(BitDepth::ThirtyTwoFloat({
                        let mut tmpv = Vec::with_capacity(data_bytes.len() / 4);
                        tmpv.extend(
                            data_bytes
                                .chunks_exact(4)
                                .map(|f| f32::from_le_bytes([f[0], f[1], f[2], f[3]])),
                        );
                        tmpv
                    })),
                    _ => Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unsupported IEEE Float bit depth",
                    )),
                },
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unsupported WAV format",
                )),
            };

            return wav_data;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Could not parse audio data",
    ))
}

fn verify_wav_file<R>(reader: &mut R) -> io::Result<riff::Chunk>
where
    R: Read + io::Seek,
{
    let wav = riff::Chunk::read(reader, 0)?;

    let form_type = wav.read_type(reader)?;

    if form_type.as_str() == "WAVE" {
        Ok(wav)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "RIFF file type not \"WAVE\"",
        ))
    }
}
