#![allow(unused_variables)]

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

/// A PNG container as described by the PNG spec
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html
#[derive(Debug)]
pub struct Png {
    // Write me!
    chunks: Box<Vec<Chunk>>,
}

impl Png {
    // Fill in this array with the correct values per the PNG spec
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    /// Creates a `Png` from a list of chunks using the correct header
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        let mut png_chunks = Box::new(Vec::new());
        let mut chunks_copy = chunks;
        let mut header: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..8 {
            header[i] = chunks_copy[0].as_bytes()[i]
        }
        png_chunks.append(&mut chunks_copy);
        Png { chunks: png_chunks }
    }

    /// Creates a `Png` from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, &'static str> {
        todo!()
    }

    /// Appends a chunk to the end of this `Png` file's `Chunk` list.
    pub fn append_chunk(&mut self, chunk: Chunk) {
        let chunk_copy = chunk;
        self.chunks.push(chunk_copy);
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and removes the first
    /// matching `Chunk` from this `Png` list of chunks.
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk, &'static str> {
        let remove = match ChunkType::from_str(chunk_type) {
            Ok(remove) => remove,
            Err(msg) => return Err(msg),
        };
        for (i, v) in self.chunks.iter().enumerate() {
            if v.chunk_type() == &remove {
                let v = match Chunk::try_from(v.as_bytes().as_slice()) {
                    Ok(res) => res,
                    Err(msg) => return Err(msg),
                };
                self.chunks.remove(i);
                return Ok(v);
            }
        }
        Err("Find no matching chunk_type")
    }

    /// The header of this PNG.
    pub fn header(&self) -> &[u8; 8] {
        &Png::STANDARD_HEADER
    }

    /// Lists the `Chunk`s stored in this `Png`
    pub fn chunks(&self) -> &[Chunk] {
        return self.chunks.as_slice();
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and returns the first
    /// matching `Chunk` from this `Png`.
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let remove = match ChunkType::from_str(chunk_type) {
            Ok(remove) => remove,
            Err(msg) => {
                eprint!("Error");
                return None;
            }
        };
        for (i, v) in self.chunks.iter().enumerate() {
            if v.chunk_type() == &remove {
                return Some(v);
            }
        }
        None
    }

    /// Returns this `Png` as a byte sequence.
    /// These bytes will contain the header followed by the bytes of all of the chunks.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut Png::STANDARD_HEADER.to_vec());
        for i in self.chunks.as_slice() {
            bytes.append(&mut i.as_bytes());
        }
        bytes
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Png, &'static str> {
        if bytes.len() < Png::STANDARD_HEADER.len() {
            return Err("Illage Input len");
        }
        let (header, bytes) = bytes.split_at(Png::STANDARD_HEADER.len());

        if header != Png::STANDARD_HEADER {
            return Err("Illage HEADER");
        }
        // split it to header and data contains
        let mut chunks = Box::new(vec![]);
        let mut i: usize = 0;
        while i < bytes.len() {
            let len = bytes.len();
            let chunk = match Chunk::try_from(&bytes[i..]) {
                Ok(chunk) => chunk,
                Err(msg) => return Err(msg),
            };
            i += chunk.length() + 12;
            chunks.push(chunk);
        }
        Ok(Png { chunks })
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
