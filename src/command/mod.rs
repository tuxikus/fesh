pub mod command_error;
pub mod command_type;
pub mod operator;

pub use command_error::CommandError;
pub use command_type::CommandType;

// CommandList stores a input after parsing
// $> echo "hello" | rg foo
pub struct CommandList {
    pub commands: Vec<Command>,
    pub operators: Vec<operator::Operator>,
}

impl CommandList {
    pub fn new(commands: Vec<Command>, operators: Vec<operator::Operator>) -> Self {
        Self {
            commands,
            operators,
        }
    }
}

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
