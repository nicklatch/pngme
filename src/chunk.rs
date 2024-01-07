#![allow(dead_code)]

use crate::chunk_type::ChunkType;
use core::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{
    fmt::{write, Display},
    io::{BufReader, Read},
};

const MAXIMUM_LENGTH: u32 = 2_147_483_647;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

// TODO: Refactor this mess
impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        //Creates a new BuffReader that reads into the buffer
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0; 4];

        // length will always be u32 (u8 * 4 == u32)
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        if length > MAXIMUM_LENGTH {
            return Err(ChunkError::InvalidLengthGT(length).into());
        }

        // read in length from buffer
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        //establish a vector the size of length, then read the chunk data into it
        let mut chunk_data = vec![0; usize::try_from(length)?];
        reader.read_exact(&mut chunk_data)?;

        //chunk_data's length should be the same as length
        if chunk_data.len() != length.try_into()? {
            return Err(
                ChunkError::InvalidLengthCmp(chunk_data.len() as u32, length.try_into()?).into(),
            );
        }

        // read in crc and test it agains our correct crc
        reader.read_exact(&mut buffer)?;
        let tried_crc = u32::from_be_bytes(buffer);
        let real_crc: u32 =
            Self::gen_u32_crc(&[&chunk_type.bytes(), chunk_data.as_slice()].concat());
        if tried_crc != real_crc {
            return Err(ChunkError::InvalidCrc(real_crc, tried_crc).into());
        }

        Ok(Chunk::new_with_all_fields(
            length, chunk_type, chunk_data, real_crc,
        ))
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.chunk_type(),
            self.data_as_string()
                .unwrap_or_else(|_| "[data]".to_string())
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data: chunk_data.clone(),
            crc: Self::gen_u32_crc(&[&chunk_type.bytes(), chunk_data.as_slice()].concat()),
        }
    }

    pub fn new_with_all_fields(
        length: u32,
        chunk_type: ChunkType,
        chunk_data: Vec<u8>,
        crc: u32,
    ) -> Self {
        Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    pub fn gen_u32_crc(bytes: &[u8]) -> u32 {
        const ALGO: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        Crc::<u32>::checksum(&ALGO, bytes)
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone()).map_err(Box::new)?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}
// TODO: IMPROVE ERROR HANDLING
type ErrorMsg = String;
#[derive(Debug)]
pub struct InvalidChunkError(ErrorMsg);

impl fmt::Display for InvalidChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Chunk: {}", self.0)
    }
}

impl std::error::Error for InvalidChunkError {}

impl InvalidChunkError {
    fn boxed(reason: ErrorMsg) -> Box<Self> {
        Box::new(Self(reason))
    }
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidLengthGT(u32),
    InvalidLengthCmp(u32, u32),
    ChunkTooSmall(u32),
    InvalidChunkType,
    InvalidCrc(u32, u32),
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChunkError::InvalidLengthGT(length) => write!(
                f,
                "Chunk length greater than 2,147,483,647: Actual {length}"
            ),
            ChunkError::InvalidLengthCmp(expected, actual) => {
                write!(f, "Expected: {expected}, Actual: {actual}")
            }
            ChunkError::InvalidChunkType => write!(f, "{}", ""),
            ChunkError::InvalidCrc(expected, actual) => write!(
                f,
                "The provided CRC of {expected} does not match the expected CRC of {actual}"
            ),
            ChunkError::ChunkTooSmall(bytes) => {
                write!(f, "Chunk is smaller than 12 bytes. Actual: {bytes}")
            }
            ChunkError::InvalidChunkType => write!(f, "Invalid Chunk Type"),
        }
    }
}

impl std::error::Error for ChunkError {}

// ----------TESTS-------------//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
