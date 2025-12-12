// this tells the compile to include the files
pub mod command; // command/*
pub mod config; // config.rs
pub mod config_parser; // config_parser.rs
pub mod fesh; // fesh.rs
pub mod file_writer; // file_writer.rs
pub mod input_parser; // input_parser.rs
pub mod input_reader; // input_reader.rs
pub mod logger; // logger.rs
pub mod prompt; // prompt.rs
pub mod util; // util.rs

use clap::Parser;
use fesh::Fesh;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn parse_args() -> Args {
    Args::parse()
}

fn main() {
    let _args = parse_args();

    let config = match config_parser::ConfigParser::new().read() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let mut fesh = Fesh::new(&config);
    fesh.run();
}
