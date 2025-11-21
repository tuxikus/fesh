pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn new(enabled: bool) -> Self {
        Logger { enabled }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn println(&self, text: String) {
        if self.enabled {
            println!("{}", text);
        }
    }
}
