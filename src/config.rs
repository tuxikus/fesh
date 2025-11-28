use crate::prompt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: prompt::Prompt,
}

