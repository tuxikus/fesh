use crate::prompt;
use serde::Deserialize;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: prompt::Prompt,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    pub readline: ReadlineConfig,
    #[serde(default)]
    pub history: HistoryConfig,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ReadlineConfig {
    #[serde(rename = "edit_mode")]
    pub edit_mode: String,
}

#[derive(Deserialize)]
pub struct HistoryConfig {
    #[serde(default = "default_history_path")]
    pub history_path: PathBuf,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        HistoryConfig {
            history_path: default_history_path(),
        }
    }
}

fn default_history_path() -> PathBuf {
    let data_home = env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME environment variable not set");
            PathBuf::from(home).join(".local/share")
        });
    data_home.join("fesh/history")
}
