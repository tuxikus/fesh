// history_writer.rs is a workaround because rustyline
// isn't writing the history :(

use std::fs::OpenOptions;
use std::io::Write;

pub struct HistoryWriter {}

impl HistoryWriter {
    pub fn new() -> Self {
        HistoryWriter {}
    }

    // FIXME: return error, not bool
    pub fn write(&self, input: &String) -> bool {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open("history.txt")
        {
            Ok(f) => f,
            Err(_) => return false,
        };

        let mut input_with_nl: String = String::from(input);
        input_with_nl.push_str("\n");

        match file.write_all(input_with_nl.as_bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
