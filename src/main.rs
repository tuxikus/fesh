mod command;
mod fesh;
mod history_writer;
mod input_parser;
mod input_reader;
mod logger;
mod mode;
mod prompt;

use clap::Parser;
use fesh::Fesh;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// d = debug, i = interactive (default)
    #[arg(short, long)]
    mode: Option<char>,
}

fn parse_args() -> Args {
    Args::parse()
}

fn main() {
    let args = parse_args();

    // TODO: use enum instead of chars
    let mode = match args.mode {
        Some(m) => m,
        None => '-',
    };

    let mut fesh = Fesh::new(String::from("fesh> "), mode);
    fesh.run();
}
