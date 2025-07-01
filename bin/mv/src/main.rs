use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "mv")]
#[command(about = "move one file to another")]
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

    fs::rename(&args.src, &args.dst)?;

    Ok(())
}
