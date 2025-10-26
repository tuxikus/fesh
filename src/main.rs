use std::io::{stdin, stdout, Write};

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
}

fn main() {
    let f = Fesh{
        prompt: String::from("fesh> ")
    };
    
    loop {
        f.print_prompt();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }
}
