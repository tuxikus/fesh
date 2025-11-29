use crate::command::command::Command;
use crate::command::operator::Operator;
use std::collections::HashMap;
#[derive(Debug)]
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

    pub fn replace_aliases(&mut self, aliases: &HashMap<String, String>) {
        for command in self.commands.iter_mut() {
            if let Some(alias_value) = aliases.get(&command.command) {
                let parts: Vec<&str> = alias_value.split_whitespace().collect();
                if !parts.is_empty() {
                    let new_cmd = parts[0].to_string();
                    let mut new_args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                    new_args.extend_from_slice(&command.args);
                    
                    let replaced = Command::new(new_cmd, new_args);
                    command.command = replaced.command;
                    command.args = replaced.args;
                    command.command_type = replaced.command_type;
                }
            }
        }
    }
}
