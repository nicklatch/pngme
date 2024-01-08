#![allow(dead_code)]

use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// A chunk type is a sequence of four bytes. Each byte must be an ASCII
/// alphabetic character. The case of each character carries a specific meaning.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType {
    bytes: [u8; 4],
}

// Converts a byte array into a `ChunkType`.
///
/// This will return an error if any of the bytes is not an ASCII alphabetic character.
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

/// Converts a string into a `ChunkType`.
///
/// This will return an error if the string is not exactly four characters long,
/// or if any of the characters is not an ASCII alphabetic character.
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

/// Formats a `ChunkType` for display.
///
/// This will output the four characters of the chunk type.
impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{}", char::from(*byte))?;
        }
        Ok(())
    }
}

/// Methods for working with a `ChunkType`.
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Returns whether the chunk type is valid.
    ///
    /// A chunk type is valid if it is exactly four characters long,
    /// each character is an ASCII alphabetic character,
    /// and the reserved bit is valid.
    pub fn is_valid(&self) -> bool {
        (self.bytes.len() == 4)
            && ChunkType::is_reserved_bit_valid(self)
            && ChunkType::all_valid_bytes(self)
    }

    /// Returns whether the chunk type is valid.
    ///
    /// A chunk type is valid if it is exactly four characters long,
    /// each character is an ASCII alphabetic character,
    /// and the reserved bit is valid.
    pub fn is_valid_byte(byte: u8) -> bool {
        matches!(byte, 65..=90 | 97..=122)
    }

    /// Returns whether the semantic bit of a byte is zero.
    ///
    /// The semantic bit is the fifth bit from the right.
    pub fn semantic_bit_is_zero(bit: u8) -> bool {
        bit & (1 << 5) == 0
    }

    /// Returns whether all bytes in the chunk type are valid characters.
    pub fn all_valid_bytes(&self) -> bool {
        self.bytes
            .iter()
            .all(|&byte| ChunkType::is_valid_byte(byte))
    }

    /// Returns whether the chunk type is critical.
    ///
    /// A chunk type is critical if the case of the first character is uppercase.
    pub fn is_critical(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[0])
    }

    /// Returns whether the chunk type is public.
    ///
    /// A chunk type is public if the case of the second character is uppercase.
    pub fn is_public(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[1])
    }

    /// Returns whether the reserved bit of the chunk type is valid.
    ///
    /// The reserved bit is valid if the case of the third character is uppercase.
    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::semantic_bit_is_zero(self.bytes[2])
    }

    /// Returns whether the chunk type is safe to copy.
    ///
    /// A chunk type is safe to copy if the case of the fourth character is lowercase.
    pub fn is_safe_to_copy(&self) -> bool {
        !Self::semantic_bit_is_zero(self.bytes[3])
    }
}

/// Represents an error that can occur when decoding a chunk type.
#[derive(Debug)]
pub enum ChunkTypeDecodeError {
    InvalidByte(u8),
    InvalidLen(usize),
    UnkownError,
}

/// Formats a `ChunkTypeDecodeError` for display.
impl fmt::Display for ChunkTypeDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidByte(byte) => write!(f, "Invalid Byte: {byte} ({byte:b}", byte = byte),
            Self::InvalidLen(len) => write!(f, "Invalid Length: Expected 4, recieved {len}"),
            Self::UnkownError => write!(f, "An unkown error has occured"),
        }
    }
}

/// Allows a `ChunkTypeDecodeError` to be treated as a standard error.
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
