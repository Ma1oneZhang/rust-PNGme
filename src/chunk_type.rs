use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
    vaild: bool,
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3], self.vaild
        )
    }
}
impl PartialEq for ChunkType {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes && self.vaild == other.vaild
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let vaild = true;
        for i in bytes {
            match i {
                65..=90 => continue,
                97..=122 => continue,
                _ => return Err("Illage input"),
            }
        }
        Ok(ChunkType { bytes, vaild })
    }
}
impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let vaild = true;
        if str.len() != 4 {
            return Err("Illega input".into());
        }
        let mut bytes: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            bytes[i] = str.as_bytes()[i] as u8;
            match str.as_bytes()[i] as u8 {
                65..=90 => continue,
                97..=122 => continue,
                _ => return Err("Illage input"),
            }
        }
        Ok(ChunkType { bytes, vaild })
    }
}
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.bytes;
    }
    pub fn is_critical(&self) -> bool {
        ((self.bytes[0] >> 5) & 1) != 1
    }
    pub fn is_public(&self) -> bool {
        ((self.bytes[1] >> 5) & 1) != 1
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        ((self.bytes[2] >> 5) & 1) != 1
    }
    pub fn is_safe_to_copy(&self) -> bool {
        ((self.bytes[3] >> 5) & 1) == 1
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    pub fn is_err(&self) -> bool {
        !self.is_reserved_bit_valid()
    }
    pub fn to_string(&self) -> String {
        std::str::from_utf8(&self.bytes).unwrap().to_owned()
    }
}
