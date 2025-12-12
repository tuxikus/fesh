use std::borrow::Cow::{self, Owned};
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use rustyline::Editor;
use rustyline::Config as RustylineConfig;
use rustyline::completion::FilenameCompleter;
use rustyline::EditMode;
use rustyline::CompletionType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Helper, Hinter, Validator};

use crate::config;
use crate::logger;
use crate::prompt;
use crate::util;

#[derive(Helper, Completer, Hinter, Validator)]
struct FeshHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for FeshHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

pub struct InputReader<'a> {
    pub logger: logger::Logger,
    readline_config: &'a config::ReadlineConfig,
    history_path: PathBuf,
}

impl<'a> InputReader<'a> {
    pub fn new(readline_config: &'a config::ReadlineConfig, history_config: &config::HistoryConfig) -> Self {
        InputReader {
            logger: logger::Logger::new(false),
            readline_config,
            history_path: history_config.history_path.clone(),
        }
    }

    pub fn readline(&self, prompt: &prompt::Prompt) -> String {
        let edit_mode = match self.readline_config.edit_mode.as_str() {
            "emacs" => EditMode::Emacs,
            "vi" => EditMode::Vi,
            _ => util::exit_with_error("invalid edit mode"),
        };

        let config = RustylineConfig::builder()
            .edit_mode(edit_mode)
            .completion_type(CompletionType::List)
            .build();

        let helper = FeshHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: HistoryHinter::new(),
        };

        let mut rl: Editor<FeshHelper, _> = match Editor::with_config(config) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("unable to invoke rustyline crate: {e}");
                exit(-1);
            }
        };

        rl.set_helper(Some(helper));

        if rl.load_history(&self.history_path).is_err() {
            self.logger.print_debug(String::from("InputReader"), format!("no previous history found"));
        }

        let readline = rl.readline(&prompt.get_colored_prompt());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                self.save_history(&mut rl);
                line
            },
            // Ctrl + d
            Err(ReadlineError::Eof) => {
                self.save_history(&mut rl);
                exit(0);
            }
            _ => "".to_string(),
        }
    }

    fn save_history<H: rustyline::Helper>(&self, rl: &mut Editor<H, rustyline::history::DefaultHistory>) {
        if let Some(parent) = self.history_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    self.logger.print_debug(String::from("InputReader"), format!("history directory cant be created: {e}"));
                    return;
                }
            }
        }
        
        if let Err(e) = rl.save_history(&self.history_path) {
            self.logger.print_debug(String::from("InputReader"), format!("history cant be saved: {e}"));
        }
    }
}
