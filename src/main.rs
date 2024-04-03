use clap::Parser;

mod args;
mod core;

fn main() {
    let parsed_args = args::MainArgs::parse();

    match parsed_args.command {
        args::Command::Compress(args) => {
            let compressed_file = core::compress(&args.file_path);
            compressed_file.save_to_file(&(args.file_path + "_compressed"))
        }
        args::Command::Decompress(args) => {
            let decompressed_file = core::decompress(&args.file_path);
            decompressed_file.save_to_file(&(args.file_path + "_decompressed"));
        }
    }
}
