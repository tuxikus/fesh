// this tells the compile to include the files
pub mod command; // command/*
pub mod fesh; // fesh.rs
pub mod file_writer; // file_writer.rs
pub mod history_writer; // history_writer.rs
pub mod input_parser; // input_parser.rs
pub mod input_reader; // input_reader.rs
pub mod logger; // logger.rs
pub mod prompt; // prompt.rs
pub mod config_parser; // config_parser.rs
pub mod config; // config.rs

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
    let _args = parse_args();

    let config = config_parser::ConfigParser::new().read().unwrap();
    let mut fesh = Fesh::new(config);
    fesh.run();
}

