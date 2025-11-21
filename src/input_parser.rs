use crate::command;
use crate::command::operator::Operator;

pub struct InputParser;

impl InputParser {
    pub fn parse(&self, input: String) -> Result<command::CommandList, command::CommandError> {
        let operator_set = ["|", ">", ">>"];
        let mut commands: Vec<command::Command> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();

        if input.is_empty() {
            return Err(command::CommandError::Empty);
        }

        let mut current_cmd: Vec<String> = Vec::new();

        let parts = input.split_whitespace();

        for part in parts {
            if operator_set.contains(&part) {
                if !current_cmd.is_empty() {
                    let cmd =
                        command::Command::new(current_cmd[0].clone(), current_cmd[1..].to_vec());
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
                current_cmd.push(part.to_string());
            }
        }

        if !current_cmd.is_empty() {
            let cmd = command::Command::new(current_cmd[0].clone(), current_cmd[1..].to_vec());
            commands.push(cmd);
        }

        Ok(command::CommandList::new(commands, operators))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::CommandType;

    #[test]
    fn test_parse_empty_input() {
        let parser = InputParser;
        let result = parser.parse(String::from(""));
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, command::CommandError::Empty);
        }
    }

    #[test]
    fn test_parse_simple_command() {
        let parser = InputParser;
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
        let parser = InputParser;
        let result = parser.parse(String::from("ls -la /tmp"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 1);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args, vec!["-la", "/tmp"]);
    }

    #[test]
    fn test_parse_builtin_exit() {
        let parser = InputParser;
        let result = parser.parse(String::from("exit"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands[0].command, "exit");
        assert_eq!(command_list.commands[0].command_type, CommandType::Builtin);
    }

    #[test]
    fn test_parse_builtin_debug() {
        let parser = InputParser;
        let result = parser.parse(String::from("+debug"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands[0].command, "+debug");
        assert_eq!(command_list.commands[0].command_type, CommandType::Builtin);
    }

    #[test]
    fn test_parse_command_with_pipe() {
        let parser = InputParser;
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
        let parser = InputParser;
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
        let parser = InputParser;
        let result = parser.parse(String::from("echo hello >> output.txt"));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 2);
        assert_eq!(command_list.operators.len(), 1);
        assert_eq!(command_list.operators[0], Operator::RedirectAppend);
    }

    #[test]
    fn test_parse_multiple_commands_with_operators() {
        let parser = InputParser;
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
        let parser = InputParser;
        let result = parser.parse(String::from("  ls   -la   /tmp  "));
        assert!(result.is_ok());
        let command_list = result.unwrap();
        assert_eq!(command_list.commands.len(), 1);
        assert_eq!(command_list.commands[0].command, "ls");
        assert_eq!(command_list.commands[0].args, vec!["-la", "/tmp"]);
    }
}
