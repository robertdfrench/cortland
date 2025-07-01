use clap::Parser;

use std::io;
use std::process;

use std::io::Write;


#[derive(Parser)]
#[command(name = "yes")]
#[command(about = "Repeat 'yes' or an expletive")]
struct Args {
    /// Path to the file to read
    #[arg(help = "The expletive to repeat")]
    expletive: Option<String>,
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    let expletive = match args.expletive {
        Some(expletive) => expletive,
        None => String::from("yes")
    };

    let mut stdout = io::stdout();
    loop {
        match writeln!(stdout, "{expletive}") {
            Err(e) => match e.kind() {
                io::ErrorKind::BrokenPipe => {
                    process::exit(0);
                },
                _ => return Err(e)
            },
            _ => {}
        }
    }
}
