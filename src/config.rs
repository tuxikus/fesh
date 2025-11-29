use crate::prompt;
use serde::Deserialize;

use std::collections::HashMap;
#[derive(Deserialize)]
pub struct Config {
    pub prompt: prompt::Prompt,
    pub aliases: HashMap<String, String>,
}