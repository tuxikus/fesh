use std::{io::{stdin, stdout, Write}, process::Command};

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

    fn read_user_input(&self) -> String {
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            eprint!("failed to read user input: {e}");
        }

        input.trim().to_string()
    }

    fn execute_command(&self, command: &String) {
        let mut child = match Command::new(command).spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to spaw child process <{command}>: {e}");
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
        let i = f.read_user_input();
        f.execute_command(&i);
    }
}
