use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Empty,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Empty => write!(f, "command input is empty"),
        }
    }
}
