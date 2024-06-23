use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "A PNG message encoder/decoder", long_about = None)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk: String,
    pub message: String,
    pub out: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}
