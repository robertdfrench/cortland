use clap::Parser;
use std::os::unix::fs;
use std::io;

#[derive(Parser)]
#[command(name = "ln")]
#[command(about = "link one file to another")]
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

    fs::symlink(&args.src, &args.dst)?;

    Ok(())
}
