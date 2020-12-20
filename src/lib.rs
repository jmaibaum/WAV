//! # WAV
//!
//! This is a crate for reading in and writing out wave files. It supports bit depths of 8, 16, and 24 bits, any number of channels, and uncompressed PCM data. Unfortunately other types of data format (e.g. compressed WAVE files) are not supported. There is also no support for any metadata chunks or any chunks other than the "fmt " and "data" chunks.
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
//! wav::write(header, data, &mut out_file)?;
//! # Ok(())
//! # }
//! ```

#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use riff;
use std::{convert::TryFrom, io::{Read, Write}};

pub mod header;
pub use header::Header;

pub mod bit_depth;
pub use bit_depth::BitDepth;

mod tuple_iterator;
use tuple_iterator::{PairIter, TripletIter};

/// Reads in the given `Read` object and attempts to extract the audio data and
/// header from it.
///
/// # Errors
///
/// This function fails under the following circumstances:
/// * Any error occurring from the `reader` parameter during reading.
/// * The data isn't RIFF data.
/// * The wave data is malformed.
/// * The wave header specifies a compressed data format.
pub fn read(reader: &mut dyn Read) -> std::io::Result<(Header, BitDepth)> {
    let (wav, _) = riff::read_chunk(reader)?;

    let mut head = Header::default();
    let mut data = BitDepth::default();

    match wav.content {
        riff::ChunkContent::List {
            form_type,
            subchunks,
        } => {
            if form_type.as_str() != "WAVE" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "RIFF file type not \"WAVE\"",
                ));
            } else {
                // Get the header from the first chunk
                for c in &subchunks {
                    // Check for `fmt ` chunk
                    if c.id.as_str() == "fmt " {
                        if let riff::ChunkContent::Subchunk(v) = &c.content {
                            if let Ok(h) = Header::try_from(v.as_slice()) {
                                head = h;
                            } else {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "WAVE \"fmt \" chunk is malformed, cannot continue",
                                ))
                            }
                        }
                    }
                }
                // Return error if not using PCM
                if head.audio_format != 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "File does not use uncompressed PCM data format",
                    ));
                }

                // Get the data from the second chunk
                for c in &subchunks {
                    // Check for `data` chunk
                    if c.id.as_str() == "data" {
                        if let riff::ChunkContent::Subchunk(v) = &c.content {
                            match head.bits_per_sample {
                                8 => {
                                    data = BitDepth::Eight(v.clone());
                                }
                                16 => {
                                    let mut i = 0;
                                    let mut sam = Vec::new();
                                    while i < v.len() {
                                        for _ in 0..head.channel_count {
                                            sam.push(i16::from_le_bytes([v[i], v[i + 1]]));
                                            i += 2;
                                        }
                                    }
                                    data = BitDepth::Sixteen(sam);
                                }
                                24 => {
                                    let mut i = 0;
                                    let mut sam = Vec::new();
                                    while i < v.len() {
                                        for _ in 0..head.channel_count {
                                            sam.push(i32::from_le_bytes([
                                                0,
                                                v[i    ],
                                                v[i + 1],
                                                v[i + 2],
                                            ]));
                                            i += 3;
                                        }
                                    }
                                    data = BitDepth::TwentyFour(sam);
                                }
                                _ => {
                                    return Err(std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        "Unsupported bit depth",
                                    ))
                                }
                            };
                        }
                    }
                }
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File not a WAVE file",
            ))
        }
    };

    if data == BitDepth::Empty {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not parse audio data",
        ));
    }

    Ok((head, data))
}

/// Writes the given wav data to the given `Write` object.
///
/// Although `track` is a borrowed value, its contents will be formatted into an
/// owned `Vec<u8>` so that it can be written to the `writer` through
/// [`riff::write_chunk`][0].
///
/// # Errors
///
/// This function fails under the following circumstances:
/// * Any error occurring from the `writer` parameter during writing.
/// * The path to the desired file destination couldn't be created.
/// * The given BitDepth is `BitDepth::Empty`.
///
/// [0]: riff::write_chunk
pub fn write(header: Header, track: &BitDepth, writer: &mut dyn Write) -> std::io::Result<()> {
    let w_id = riff::ChunkId::new("WAVE").unwrap();

    let h_id = riff::ChunkId::new("fmt ").unwrap();
    let h_vec: [u8; 16] = header.into();
    let h_dat = riff::Chunk::new_data(h_id, Vec::from(&h_vec[0..16]));

    let d_id = riff::ChunkId::new("data").unwrap();
    let d_vec = match track {
        BitDepth::Eight(v) => v.clone(),
        BitDepth::Sixteen(v) => v.iter()
            .flat_map(
                |s| {
                    let v = s.to_le_bytes();
                    PairIter::new((v[0], v[1]))
                }
            )
            .collect::<Vec<_>>(),
        BitDepth::TwentyFour(v) => v.iter()
            .flat_map(
                |s| {
                    let v = s.to_le_bytes().split_at(1).1.to_owned();
                    TripletIter::new((v[0], v[1], v[2]))
                }
            )
            .collect::<Vec<_>>(),
        _ => return Err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Empty audio data given",
            )
        ),
    };
    let d_dat = riff::Chunk::new_data(d_id, d_vec);

    let r = riff::Chunk::new_riff(w_id, vec![h_dat, d_dat]);

    riff::write_chunk(writer, &r)?;

    Ok(())
}
