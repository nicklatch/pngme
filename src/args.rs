use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Commands {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Debug, Parser)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
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
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
