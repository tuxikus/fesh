use std::path;
use std::process::exit;
use std::process::Command as SysCommand;
use std::process::Stdio;

use crate::command::command;
use crate::command::command_list::CommandList;
use crate::command::command_type::CommandType;
use crate::command::operator::Operator;
use crate::file_writer;
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
    file_writer: file_writer::FileWriter,
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
            input_parser: input_parser::InputParser::new(),
            file_writer: file_writer::FileWriter::new(),
        }
    }

    fn toggle_logger(&mut self) {
        self.logger.print_debug(String::from("Fesh"), format!("toggle debug logging"));
        self.input_reader.logger.toggle_debug();
        self.input_parser.logger.toggle_debug();
        self.input_reader.history_writer.logger.toggle_debug();
        self.file_writer.logger.toggle_debug();
        self.logger.toggle_debug();
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
    fn check_first_builtin(&mut self, command_list: &CommandList) -> bool {
        self.logger.print_debug(String::from("Fesh"), format!("checking for builtin"));
        if let Some(first_command) = command_list.commands.first() {
            if first_command.command_type == CommandType::Builtin {
                return self.execute_buitin(first_command.clone());
            }
            return false;
        }
        return false;
    }

    fn execute_buitin(&mut self, command_input: command::Command) -> bool {
        self.logger.print_debug(String::from("Fesh"), format!("executing builtin: {}", command_input.command));
        match command_input.command.as_str() {
            "exit" => {
                exit(0);
            }
            "+debug" => {
                self.toggle_logger();
                return true;
            }
            _ => false,
        }
    }

    pub fn execute_command_list(&mut self, command_list: CommandList) {
        self.logger.print_debug(String::from("Fesh"), format!("executing command list: {:?}", command_list));
        let is_builtin = self.check_first_builtin(&command_list);
        if is_builtin { return }

        let mut prev_stdout: Option<Stdio> = None;
        let mut children = Vec::new();
        let mut skip_next = false;

        for (i, command) in command_list.commands.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let operator = command_list.operators.get(i);

            let mut cmd = SysCommand::new(&command.command);
            if !command.args.is_empty() {
                cmd.args(&command.args);
            }

            if let Some(stdin) = prev_stdout.take() {
                cmd.stdin(stdin);
            } else {
                cmd.stdin(Stdio::inherit());
            }

            match operator {
                Some(Operator::Pipe) => {
                    self.logger.print_debug(String::from("Fesh"), format!("executing pipe"));
                    cmd.stdout(Stdio::piped());
                }
                Some(Operator::RedirectOverwrite) => {
                    let path = path::Path::new(&command_list.commands.get(i + 1).unwrap().command);
                    self.logger.print_debug(String::from("Fesh"), format!("executing redirect overwrite to <{}>", path.display()));
                    cmd.stdout(Stdio::piped());
                    let output = match cmd.output() {
                        Ok(o) => o,
                        Err(e) => {
                            self.logger.print_error(format!("error while redirect: {e:?}"));
                            return;
                        }
                    };
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if let Err(e) = self.file_writer.overwrite_file(path, &output_str) {
                        self.logger.print_error(format!("error writing to file: {e:?}"));
                        return;
                    }

                    skip_next = true;
                    continue;
                }
                Some(Operator::RedirectAppend) => {
                    let path = path::Path::new(&command_list.commands.get(i + 1).unwrap().command);
                    self.logger.print_debug(String::from("Fesh"), format!("executing redirect append to <{}>", path.display()));
                    cmd.stdout(Stdio::piped());
                    let output = match cmd.output() {
                        Ok(o) => o,
                        Err(e) => {
                            self.logger.print_error(format!("error while redirect append: {e:?}"));
                            return;
                        }
                    };
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if let Err(e) = self.file_writer.append_to_file(path, &output_str) {
                        self.logger.print_error(format!("error appending to file: {e:?}"));
                        return;
                    }

                    skip_next = true;
                    continue;
                }
                None => {
                    cmd.stdout(Stdio::inherit());
                }
            }

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    self.logger.print_error(format!("failed to spawn child process <{}>: {e}", command.command));
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
                self.logger.print_error(format!("failed to wait for child process: {e}"));
            }
        }
    }
}
