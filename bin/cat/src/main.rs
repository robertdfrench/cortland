use clap::Parser;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "cat")]
#[command(about = "A simple cat program that prints file contents")]
struct Args {
    /// Path to the file to read
    #[arg(help = "The file to read and print")]
    file: String,
}

fn main() {
    let args = Args::parse();
    
    match fs::read_to_string(&args.file) {
        Ok(contents) => print!("{}", contents),
        Err(e) => {
            eprintln!(
                "Error reading file '{}': {}", args.file, e);
            process::exit(1);
        }
    }
}
