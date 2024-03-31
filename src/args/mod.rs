use clap::{Parser, Subcommand};

use self::{compress::CompressArgs, decompress::DecompressArgs};

mod compress;
mod decompress;

#[derive(Parser)]
pub struct MainArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Compress(CompressArgs),
    Decompress(DecompressArgs),
}
