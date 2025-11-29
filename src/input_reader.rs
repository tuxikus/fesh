use std::process::exit;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::history_writer;
use crate::logger;
use crate::prompt;

pub struct InputReader {
    pub history_writer: history_writer::HistoryWriter,
    pub logger: logger::Logger,
}

impl InputReader {
    pub fn new() -> Self {
        InputReader {
            history_writer: history_writer::HistoryWriter::new(),
            logger: logger::Logger::new(false),
        }
    }

    pub fn readline(&self, prompt: &prompt::Prompt) -> String {
        let mut rl = match DefaultEditor::new() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("unable to invoke rustyline crate: {e}");
                exit(-1);
            }
        };

        if rl.load_history("history.txt").is_err() {
            self.logger.print_debug(String::from("InputReader"), format!("no previous history found"));
        }

        let readline = rl.readline(&prompt.get_colored_prompt());
        match readline {
            Ok(line) => line,
            // Ctrl + d
            Err(ReadlineError::Eof) => {
                match rl.save_history("history.txt") {
                    Ok(_) => (),
                    Err(e) => eprintln!("+history cant be saved: {e}"),
                }
                exit(0);
            }
            _ => "".to_string(),
        }
    }
}
