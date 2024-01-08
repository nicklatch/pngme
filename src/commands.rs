#![allow(dead_code)]

use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

pub fn run(command: PngMeArgs) -> Result<()> {
    match command {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let input = fs::read(&args.file_path)?;
    let output = match &args.output_file {
        Some(o) => o,
        None => &args.file_path,
    };

    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
    );

    let mut png: Png = Png::try_from(input.as_slice())?;

    png.append_chunk(chunk);

    fs::write(output, png.as_bytes())?;

    println!("Secret successfully encoded!");

    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let input = fs::read(&args.file_path)?;
    let png: Png = Png::try_from(input.as_slice())?;
    let chunk = png.chunk_by_type(args.chunk_type.as_str());

    if let Some(c) = chunk {
        println!("{c}")
    }

    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let input = fs::read(&args.file_path)?;
    let mut png: Png = Png::try_from(input.as_slice())?;
    match png.remove_chunk(args.chunk_type.as_str()) {
        Ok(chunk) => {
            fs::write(&args.file_path, png.as_bytes())?;
            println!("Removed chunk: {}", chunk);
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let input = fs::read(args.file_path)?;
    let png = Png::try_from(input.as_slice())?;

    png.chunks().iter().for_each(|chunk| println!("{chunk}"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_encode() {
        let args = EncodeArgs {
            file_path: PathBuf::from("test.png"),
            chunk_type: String::from("tEXt"),
            message: String::from("Test message"),
            output_file: None,
        };
        assert!(encode(args).is_ok());
    }

    #[test]
    fn test_decode() {
        let args = DecodeArgs {
            file_path: PathBuf::from("test.png"),
            chunk_type: String::from("tEXt"),
        };
        assert!(decode(args).is_ok());
    }

    #[test]
    fn test_remove() {
        let args = RemoveArgs {
            file_path: PathBuf::from("test.png"),
            chunk_type: String::from("tEXt"),
        };
        assert!(remove(args).is_ok());
    }

    #[test]
    fn test_print() {
        let args = PrintArgs {
            file_path: PathBuf::from("test.png"),
        };
        assert!(print(args).is_ok());
    }
}
