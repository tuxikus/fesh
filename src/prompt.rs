pub struct Prompt(String);

impl Prompt {
    pub fn new(s: String) -> Self {
        Prompt(s)
    }

    pub fn get(&self) -> &String {
        &self.0
    }
}
