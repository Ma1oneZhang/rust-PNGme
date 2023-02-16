use crc::{Algorithm, Crc, CRC_32_ISO_HDLC};

use super::chunk_type::ChunkType;
use std::{
    fmt::Display,
    process::{self, Termination},
};
impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() <= 12 {
            return Err("Illage input");
        }
        let length: u32 = ((value[0] as u32) << 24)
            + ((value[1] as u32) << 16)
            + ((value[2] as u32) << 8)
            + value[3] as u32;
        let chunk_type: ChunkType =
            match ChunkType::try_from([value[4], value[5], value[6], value[7]]) {
                Ok(chunk_type) => chunk_type,
                Err(msg) => {
                    eprintln!("{msg}");
                    process::exit(1);
                }
            };
        let mut crc: u32 = 0;
        for i in value.len() - 4..=value.len() - 1 {
            crc <<= 8;
            crc += value[i] as u32;
        }
        if crc != checksum_32(&CRC_32_ISO_HDLC, &value[4..=value.len() - 5]) {
            return Err("Illega checksum");
        }
        let data = Box::new(value[8..value.len() - 4].to_vec());
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
pub struct Chunk {
    length: u32,
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
        assert!(crc == 2882656334);
        let mut data = Box::new(data);
        let length = data.as_slice().len() as u32;
        Chunk {
            length,
            chunk_type,
            data: data,
            crc: crc,
        }
    }
    pub fn length(&self) -> u32 {
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
        *self.data.clone()
    }
}
