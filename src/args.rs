use clap::Parser;
use std::path::PathBuf;

///A CLI Application to Embed Messages Into A PNG File!
#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct Commands {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

/// Represents the different subcommands that the application can accept.
/// Each variant corresponds to a different operation that can be performed on a PNG file.
#[derive(Debug, Parser)]
#[command(about, long_about)]
pub enum PngMeArgs {
    /// <FILE_PATH> | Represents the "encode" subcommand, which is used to embed a message into a PNG file.
    Encode(EncodeArgs),
    /// <FILE_PATH> | Represents the "decode" subcommand, which is used to extract a message from a PNG file.
    Decode(DecodeArgs),
    /// <FILE_PATH> | Represents the "remove" subcommand, which is used to remove a message from a PNG file.
    Remove(RemoveArgs),
    /// <FILE_PATH> | Represents the "print" subcommand, which is used to print the chunks of a PNG file.
    Print(PrintArgs),
}

/// Represents the arguments for the "encode" subcommand.
#[derive(Debug, Parser)]
pub struct EncodeArgs {
    /// The path to the PNG file to encode a message into.
    pub file_path: PathBuf,
    /// The type of the chunk to encode the message into.
    pub chunk_type: String,
    /// The message to encode into the PNG file.
    pub message: String,
    /// The path to the output file. If not provided, the original file will be overwritten.
    pub output_file: Option<PathBuf>,
}

/// Represents the arguments for the "decode" subcommand.
#[derive(Debug, Parser)]
pub struct DecodeArgs {
    /// The type of the chunk to decode the message from.
    pub chunk_type: String,
    /// The path to the PNG file to decode a message from.
    pub file_path: PathBuf,
}

/// Represents the arguments for the "remove" subcommand.
#[derive(Debug, Parser)]
pub struct RemoveArgs {
    /// The type of the chunk to remove the message from.
    pub chunk_type: String,
    /// The path to the PNG file to remove a message from.
    pub file_path: PathBuf,
}

/// Represents the arguments for the "print" subcommand.
#[derive(Debug, Parser)]
pub struct PrintArgs {
    /// The path to the PNG file to print the chunks from.
    pub file_path: PathBuf,
}
