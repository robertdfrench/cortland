use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "cp")]
#[command(about = "cp one file to another")]
struct Args {
    /// Path to the source file
    #[arg(help = "source file")]
    src: String,

    /// Path to the dest file
    #[arg(help = "dest file")]
    dst: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    fs::copy(&args.src, &args.dst)?;

    Ok(())
}
