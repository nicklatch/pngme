use std::path::PathBuf;

use clap::Parser;

/// A cli tool to embed messages into a PNG file
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Debug, Parser)]
pub enum PngMeArgs {
    /// Encodes the message in a specific PNG file
    Encode(EncodeArgs),
    /// Decodes the message from a specific PNG file
    Decode(DecodeArgs),
    /// Remove the message from a specific PNG file
    Remove(RemoveArgs),
    /// Print the PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Debug, Parser)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct DecodeArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Input PNG file path
    pub file_path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Input PNG file path
    pub file_path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct PrintArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
}
