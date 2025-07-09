use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "mkdir")]
#[command(about = "create a directory")]
struct Args {
    /// Path to the directory to create
    #[arg(help = "directory to create")]
    dir: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    fs::create_dir(&args.dir)?;

    Ok(())
}