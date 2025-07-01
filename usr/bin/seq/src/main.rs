use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(name = "seq")]
#[command(about = "Print numbers in a sequence")]
struct Args {
    /// Path to the source file
    #[arg(help = "source file")]
    a: i32,

    /// Path to the dest file
    #[arg(help = "dest file")]
    b: i32,
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.a < args.b {
        for x in args.a..(args.b + 1) {
            println!("{x}");
        }
    } else {
        for x in args.a..(args.b + 1) {
            let y = (args.a - x) + args.b;
            println!("{y}");
        }
    }

    Ok(())
}
