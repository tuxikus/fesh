use std::process::Command as SysCommand;
use std::process::Stdio;
use std::process::exit;

use crate::command::command;
use crate::command::command_list::CommandList;
use crate::command::command_type::CommandType;
use crate::command::operator::Operator;
use crate::input_parser;
use crate::input_reader;
use crate::logger;
use crate::mode;
use crate::prompt;

pub struct Fesh {
    mode: mode::Mode,
    prompt: prompt::Prompt,
    logger: logger::Logger,
    input_reader: input_reader::InputReader,
    input_parser: input_parser::InputParser,
}

impl Fesh {
    pub fn new(prompt: String, mode_char: char) -> Self {
        let mode = match mode_char {
            'd' => mode::Mode::Debug,
            _ => mode::Mode::Interactive,
        };

        let logger_enabled = if mode == mode::Mode::Debug {
            true
        } else {
            false
        };

        Fesh {
            mode,
            prompt: prompt::Prompt::new(prompt),
            logger: logger::Logger::new(logger_enabled),
            input_reader: input_reader::InputReader::new(),
            input_parser: input_parser::InputParser {}, // TODO: add new fn
        }
    }

    pub fn run(&mut self) {
        loop {
            let input: String = self.input_reader.readline(self.prompt.get().clone());
            let command_list: CommandList = match self.input_parser.parse(input) {
                Ok(c) => c,
                Err(_) => continue,
            };
            self.execute_command_list(command_list);
        }
    }

    // currently only first command can be a builtin
    fn check_first_builtin(&self, command_list: &CommandList) {
        if let Some(first_command) = command_list.commands.first() {
            if first_command.command_type == CommandType::Builtin {
                self.execute_buitin(first_command.clone());
                return;
            }
        }
    }

    fn execute_buitin(&self, command_input: command::Command) {
        match command_input.command.as_str() {
            "exit" => {
                exit(0);
            }
            "+debug" => {
                println!("+prompt: {}", self.prompt.get());
            }
            _ => {}
        }
    }

    fn execute_external(&self, command_input: command::Command) {
        println!("running external...");
        let cmd = command_input.command;
        let args = command_input.args;

        let mut child = match SysCommand::new(&cmd).args(&args).spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to spawn child process <{cmd}>: {e}");
                return;
            }
        };

        if let Err(e) = child.wait() {
            eprintln!("failed to wait for child process: {e}")
        }
    }

    fn check_no_operators(&self, command_list: &CommandList) {
        if command_list.operators.is_empty() {
            println!("no operators");
            if let Some(command) = command_list.commands.first() {
                self.execute_external(command.clone());
            }
            return;
        }
    }

    pub fn execute_command_list(&self, command_list: CommandList) {
        self.check_first_builtin(&command_list);
        //self.check_no_operators(&command_list);

        let mut prev_stdout: Option<Stdio> = None;
        let mut children = Vec::new();

        for (i, command) in command_list.commands.iter().enumerate() {
            let operator = command_list.operators.get(i);

            let mut cmd = SysCommand::new(&command.command);
            if !command.args.is_empty() {
                cmd.args(&command.args);
            }

            // Set stdin from previous command's stdout (for pipes)
            if let Some(stdin) = prev_stdout.take() {
                cmd.stdin(stdin);
            } else {
                cmd.stdin(Stdio::inherit());
            }

            match operator {
                Some(Operator::Pipe) => {
                    cmd.stdout(Stdio::piped());
                }
                None => {
                    cmd.stdout(Stdio::inherit());
                }
                _ => {
                    todo!();
                }
            }

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("failed to spawn child process <{}>: {e}", command.command);
                    return;
                }
            };

            if operator == Some(&Operator::Pipe) {
                if let Some(stdout) = child.stdout.take() {
                    prev_stdout = Some(Stdio::from(stdout));
                }
            }

            children.push(child);
        }

        for mut child in children {
            if let Err(e) = child.wait() {
                eprintln!("failed to wait for child process: {e}");
            }
        }
    }
}
