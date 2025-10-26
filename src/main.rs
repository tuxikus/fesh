use std::{fmt, io::{stdin, stdout, Write}, process::Command};

struct CommandInput {
    command: String,
    args: Vec<String>,
}

enum CommandInputError {
    Empty,
    UnableToParse,
}

impl fmt::Display for CommandInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandInputError::Empty => write!(f, "command input is empty"),
            CommandInputError::UnableToParse => write!(f, "unable to parse input"),
        }
    }
}

struct Fesh {
    prompt: String,
}

impl Fesh {
    fn print_prompt(&self) {
        let prompt: &[u8] = self.prompt.as_bytes();
        if let Err(e) = stdout().write_all(prompt) {
            eprintln!("failed to write prompt: {e}");
            return;
        };

        if let Err(e) = stdout().flush() {
            eprintln!("failed to flush stdout: {e}")
        };
    }

    fn read_user_input(&self) -> CommandInput {
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprint!("failed to read user input: {e}");
        }
        
        match self.parse_input(input.trim().to_string()) {
            Ok(command_input) => command_input,
            Err(e) => {
                eprintln!("failed to parse user input: {e}");
                return CommandInput { command: "".to_string(), args: vec![] };
            }
        }
    }

    fn parse_input(&self, input: String) -> Result<CommandInput, CommandInputError> {
        let cmd: String;
        let mut args: Vec<String> = Vec::new();

        if input.is_empty() {
            return Err(CommandInputError::Empty);
        }

        // FIXME: remove go code :)
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(CommandInputError::UnableToParse);
        }
        
        cmd = parts[0].to_string();

        for i in 1..parts.len() {
            args.push(parts[i].to_string());
        }
        
        return Ok(CommandInput { command: cmd, args: args });
    }

    fn execute_command(&self, command_input: CommandInput) {
        let cmd = command_input.command;
        let args = command_input.args;
        
        let mut child = match Command::new(&cmd)
            .args(args)
            .spawn()
             {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to spaw child process <{cmd}>: {e}");
                return;
            }
        };
        
        if let Err(e) = child.wait() {
            eprintln!("failed to wait for child process: {e}")
        }
    }
}

fn main() {
    let f = Fesh {
        prompt: String::from("fesh> "),
    };

    loop {
        f.print_prompt();
        let command_input = f.read_user_input();
        f.execute_command(command_input);
    }
}
