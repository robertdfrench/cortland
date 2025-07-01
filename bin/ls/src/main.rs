use clap::Parser;

use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "ls")]
#[command(about = "list the contents of the named directory")]
struct Args {
    /// Path to the file to read
    #[arg(help = "The dir to list")]
    dirname: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let dirname = match args.dirname {
        Some(dirname) => dirname,
        None => String::from(".")
    };

    let entries = fs::read_dir(dirname)?
        .map(|f| f.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for e in entries {
        println!("{}", e.display());
    }

    Ok(())
}
