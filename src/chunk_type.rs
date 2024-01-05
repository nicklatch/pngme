#![allow(dead_code)]

use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in bytes.iter() {
            if !ChunkType::is_valid_byte(*byte) {
                return Err(Box::new(ChunkTypeDecodeError::InvalidByte(*byte)));
            }
        }
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeDecodeError::InvalidLen(s.len())));
        }

        let mut temp: [u8; 4] = [0; 4];

        for (i, byte) in s.as_bytes().iter().enumerate() {
            if ChunkType::is_valid_byte(*byte) {
                temp[i] = *byte
            } else {
                return Err(Box::new(ChunkTypeDecodeError::InvalidByte(*byte)));
            }
        }

        Ok(ChunkType { bytes: temp })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{}", char::from(*byte))?;
        }
        Ok(())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        (self.bytes.len() == 4) && ChunkType::is_reserved_bit_valid(self) && ChunkType::all_valid_bytes(self)
    }

    fn is_valid_byte(byte: u8) -> bool {
        matches!(byte, 65..=90 | 97..=122)
    }

    fn semantic_bit_is_zero(bit: u8) -> bool {
        bit & (1 << 5) == 0
    }

    fn all_valid_bytes(&self) -> bool {
        self.bytes.iter().all(|&byte| ChunkType::is_valid_byte(byte))
    }

    fn is_critical(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[0])
    }

    fn is_public(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[1])
    }

    fn is_reserved_bit_valid(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[2])
    }

    fn is_safe_to_copy(&self) -> bool {
        !Self::semantic_bit_is_zero(self.bytes[3])
    }
}

#[derive(Debug)]
pub enum ChunkTypeDecodeError {
    InvalidByte(u8),
    InvalidLen(usize),
}

impl fmt::Display for ChunkTypeDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidByte(byte) => write!(f, "Invalid Byte: {byte} ({byte:b}", byte = byte),
            Self::InvalidLen(len) => write!(f, "Invalid Length: Expected 4, recieved {len}"),
        }
    }
}

impl Error for ChunkTypeDecodeError {}


// ----------TESTS-------------//

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
