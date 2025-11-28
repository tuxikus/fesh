use colored::Colorize;

pub struct Logger {
    pub debug_enabled: bool,
}

impl Logger {
    pub fn new(enabled: bool) -> Self {
        Logger { debug_enabled: enabled }
    }

    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled;
    }

    pub fn print_error(&self, text: String) {
        eprintln!("[{}] {}", "ERROR".red(), text);
    }

    pub fn print_debug(&self, part: String, text: String) {
        if self.debug_enabled {
            println!("+[{}][{}] {}", "DEBUG".green(), part.magenta(), text);
        }
    }
}