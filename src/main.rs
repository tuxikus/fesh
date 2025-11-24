// this tells the compile to include the files
pub mod command; // command/*
pub mod fesh; // fesh.rs
pub mod history_writer; // history_writer.rs
pub mod input_parser; // input_parser.rs
pub mod input_reader; // input_reader.rs
pub mod logger; // logger.rs
pub mod mode; // mode.rs
pub mod prompt; // prompt.rs

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
