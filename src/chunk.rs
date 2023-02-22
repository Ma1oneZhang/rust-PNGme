#![allow(unused_variables)]
use crc::{Algorithm, Crc, CRC_32_ISO_HDLC};

use super::chunk_type::ChunkType;

use std::{
    fmt::Display,
    process::{self, Termination},
};

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Illage input");
        }

        // split length and data
        let (length, value) = value.split_at(4);
        let length = u32::from_be_bytes(match length.try_into() {
            Ok(res) => res,
            Err(_) => return Err("try from slice error"),
        }) as usize
            + 8;
        // 8 means ChunkType and CRC as u32 + u32

        // split data to single trunk
        let (value, _) = value.split_at(length as usize);
        // see first 4 u8 as chunk_type
        let chunk_type: [u8; 4] = (&value[..4]).try_into().unwrap();
        let chunk_type: ChunkType = match ChunkType::try_from(chunk_type) {
            Ok(chunk_type) => chunk_type,
            Err(msg) => {
                eprintln!("{msg}");
                return Err(msg);
            }
        };

        // split checksum from u8 slice
        let (value, crc) = value.split_at(value.len() - 4);
        let crc: u32 = u32::from_be_bytes(crc.try_into().unwrap());
        // check the checksum is current
        if crc != checksum_32(&CRC_32_ISO_HDLC, value.as_ref()) {
            return Err("Illega checksum");
        }
        let length = length - 8;
        // transfer to heap data
        let data = Box::new(value[4..].to_vec());

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl Termination for Chunk {
    fn report(self) -> process::ExitCode {
        drop(self.data);
        0.into()
    }
}
#[derive(Debug)]
pub struct Chunk {
    length: usize,
    chunk_type: ChunkType,
    data: Box<Vec<u8>>,
    crc: u32,
}
fn checksum_32(algo: &'static Algorithm<u32>, bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&algo);
    crc.checksum(bytes)
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut cloned_data = data.clone();
        let mut chunk_type_byte = chunk_type.bytes().to_vec();
        chunk_type_byte.append(&mut cloned_data);
        let crc = checksum_32(&CRC_32_ISO_HDLC, &chunk_type_byte.as_slice());
        let data = Box::new(data);
        let length = data.len();
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data.as_slice()
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, &'static str> {
        match String::from_utf8(*self.data.clone()) {
            Ok(str) => Ok(str),
            Err(msg) => Err("Error from data as string"),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut (self.length.clone() as u32).to_be_bytes().to_vec());
        bytes.append(&mut self.chunk_type.bytes().to_vec().clone());
        bytes.append(&mut self.data().to_vec().clone());
        bytes.append(&mut self.crc.clone().to_be_bytes().to_vec());
        bytes
    }
}
