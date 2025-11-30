use crate::prompt;
use serde::Deserialize;

use std::collections::HashMap;
#[derive(Deserialize)]
pub struct Config {
    pub prompt: prompt::Prompt,
    pub aliases: HashMap<String, String>,
    pub readline: ReadlineConfig
}

#[derive(Deserialize)]
pub struct ReadlineConfig {
    #[serde(rename = "edit_mode")]
    pub edit_mode: String,
}