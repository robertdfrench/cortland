use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "rm")]
#[command(about = "remove a file")]
struct Args {
    /// Path to the source file
    #[arg(help = "doomed file")]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    fs::remove_file(&args.file)?;

    Ok(())
}
