use crate::command::command_type::CommandType;

#[derive(Clone, PartialEq)]
pub struct Command {
    pub command_type: CommandType,
    pub command: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(command: String, args: Vec<String>) -> Self {
        let command_type = match command.as_str() {
            "exit" | "+debug" => CommandType::Builtin,
            _ => CommandType::External,
        };

        Self {
            command_type,
            command,
            args,
        }
    }
}
