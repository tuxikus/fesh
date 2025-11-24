use crate::command::command::Command;
use crate::command::operator::Operator;

pub struct CommandList {
    pub commands: Vec<Command>,
    pub operators: Vec<Operator>,
}

impl CommandList {
    pub fn new(commands: Vec<Command>, operators: Vec<Operator>) -> Self {
        Self {
            commands,
            operators,
        }
    }
}
