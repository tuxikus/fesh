// history_writer.rs is a workaround because rustyline
// isn't writing the history :(

use std::fs::OpenOptions;
use std::io::Write;

use crate::logger;

pub struct HistoryWriter {
    pub history_file: String,  
    pub logger: logger::Logger,
}

impl HistoryWriter {
    pub fn new() -> Self {
        HistoryWriter {
            history_file: String::from("history.txt"),
            logger: logger::Logger::new(false),
        }
    }

    // FIXME: return error, not bool
    pub fn write(&self, input: &String) -> bool {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.history_file)
        {
            Ok(f) => f,
            Err(_) => return false,
        };

        self.logger.print_debug(String::from("HistoryWriter"), format!("writing <{input}> to <{}>", self.history_file));

        let mut input_with_nl: String = String::from(input);
        input_with_nl.push_str("\n");

        match file.write_all(input_with_nl.as_bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
