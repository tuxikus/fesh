use std::env;

use crate::command::command::Command;
use crate::command::command_error::CommandError;
use crate::command::command_list::CommandList;
use crate::command::operator::Operator;
use crate::logger;

pub struct InputParser {
    pub logger: logger::Logger,
}

impl InputParser {
    pub fn new() -> Self {
        InputParser {
            logger: logger::Logger::new(false),
        }
    }

    pub fn parse(&self, input: String) -> Result<CommandList, CommandError> {
        let operator_set = ["|", ">", ">>"];
        let mut commands: Vec<Command> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();

        if input.is_empty() {
            self.logger.print_debug(String::from("InputParser"), format!("input is empty"));
            return Err(CommandError::Empty);
        }

        let mut current_cmd: Vec<String> = Vec::new();

        let parts = input.split_whitespace();
        for part in parts {
            if operator_set.contains(&part) {
                if !current_cmd.is_empty() {
                    let cmd = Command::new(current_cmd[0].clone(), current_cmd[1..].to_vec());
                    commands.push(cmd);
                    current_cmd.clear();
                }
                let op = match part {
                    "|" => Operator::Pipe,
                    ">" => Operator::RedirectOverwrite,
                    ">>" => Operator::RedirectAppend,
                    _ => continue,
                };
                operators.push(op);
            } else {
                current_cmd.push(Self::expand(part));
            }
        }

        if !current_cmd.is_empty() {
            let cmd = Command::new(current_cmd[0].clone(), current_cmd[1..].to_vec());
            commands.push(cmd);
        }

        self.logger.print_debug(String::from("InputParser"), format!("commands: {:?}", commands));
        self.logger.print_debug(String::from("InputParser"), format!("operators: {:?}", operators));
        Ok(CommandList::new(commands, operators))
    }

    fn expand(input: &str) -> String {
        let expanded = Self::expand_tilde(input);
        Self::expand_env_vars(&expanded)
    }

    fn expand_tilde(input: &str) -> String {
        if input == "~" {
            env::var("HOME").unwrap_or_else(|_| input.to_string())
        } else if input.starts_with("~/") {
            if let Ok(home) = env::var("HOME") {
                format!("{}{}", home, &input[1..])
            } else {
                input.to_string()
            }
        } else {
            input.to_string()
        }
    }

    fn expand_env_vars(input: &str) -> String {
        let mut result = input.to_string();
        let mut i = 0;

        while let Some(start) = result[i..].find('$') {
            let start = start + i;
            let rest = &result[start + 1..];

            let end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(rest.len());

            if end == 0 {
                i = start + 1;
                continue;
            }

            let var_name = &rest[..end];
            if let Ok(value) = env::var(var_name) {
                result = format!("{}{}{}", &result[..start], value, &rest[end..]);
                i = start + value.len();
            } else {
                i = start + 1 + end;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::command_type::CommandType;

    #[test]
    fn test_parse_empty_input() {
        let parser = InputParser::new();
        let result = parser.parse(String::from(""));
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, CommandError::Empty);
        }
    }

    #[test]
    fn test_parse_simple_command() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("ls"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 1);
        assert_eq!(command_list.operators.len(), 0);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args.len(), 0);
        assert_eq!(command_list.commands[0].command_type, CommandType::External);
    }

    #[test]
    fn test_parse_command_with_args() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("ls -la /tmp"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 1);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args, vec!["-la", "/tmp"]);
    }

    #[test]
    fn test_parse_builtin_exit() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("exit"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands[0].command, "exit");
        assert_eq!(command_list.commands[0].command_type, CommandType::Builtin);
    }

    #[test]
    fn test_parse_builtin_debug() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("+debug"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands[0].command, "+debug");
        assert_eq!(command_list.commands[0].command_type, CommandType::Builtin);
    }

    #[test]
    fn test_parse_command_with_pipe() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("ls | grep test"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 2);
        assert_eq!(command_list.operators.len(), 1);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[1].command, "grep");
        assert_eq!(command_list.commands[1].args, vec!["test"]);
        assert_eq!(command_list.operators[0], Operator::Pipe);
    }

    #[test]
    fn test_parse_command_with_redirect_overwrite() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("echo hello > output.txt"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 2);
        assert_eq!(command_list.operators.len(), 1);
        assert_eq!(command_list.commands[0].command, "echo");
        assert_eq!(command_list.commands[0].args, vec!["hello"]);
        assert_eq!(command_list.commands[1].command, "output.txt");
        assert_eq!(command_list.operators[0], Operator::RedirectOverwrite);
    }

    #[test]
    fn test_parse_command_with_redirect_append() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("echo hello >> output.txt"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 2);
        assert_eq!(command_list.operators.len(), 1);
        assert_eq!(command_list.operators[0], Operator::RedirectAppend);
    }

    #[test]
    fn test_parse_multiple_commands_with_operators() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("ls -la | grep test | wc -l"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 3);
        assert_eq!(command_list.operators.len(), 2);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args, vec!["-la"]);
        assert_eq!(command_list.commands[1].command, "grep");
        assert_eq!(command_list.commands[1].args, vec!["test"]);
        assert_eq!(command_list.commands[2].command, "wc");
        assert_eq!(command_list.commands[2].args, vec!["-l"]);
        assert_eq!(command_list.operators[0], Operator::Pipe);
        assert_eq!(command_list.operators[1], Operator::Pipe);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let parser = InputParser::new();
        let result = parser.parse(String::from("  ls   -la   /tmp  "));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 1);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args, vec!["-la", "/tmp"]);
    }
}
