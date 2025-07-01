use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "touch")]
#[command(about = "Create an empty file")]
struct Args {
    /// Path to the file to read
    #[arg(help = "The path to create")]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    fs::File::create(args.file)?;
    Ok(())
}
