use colored::Colorize;
use serde::Deserialize;

use std::path::PathBuf;
use std::env;

#[derive(Deserialize)]
pub struct Prompt {
    #[serde(rename = "text")]
    pub prompt: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: colored::Color,
    pub show_cwd: bool,
    #[serde(rename = "show_username")]
    pub show_user: bool,
    pub show_branch: bool,
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<colored::Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "black" => Ok(colored::Color::Black),
        "red" => Ok(colored::Color::Red),
        "green" => Ok(colored::Color::Green),
        "yellow" => Ok(colored::Color::Yellow),
        "blue" => Ok(colored::Color::Blue),
        "magenta" => Ok(colored::Color::Magenta),
        "cyan" => Ok(colored::Color::Cyan),
        "white" => Ok(colored::Color::White),
        "bright black" | "bright_black" => Ok(colored::Color::BrightBlack),
        "bright red" | "bright_red" => Ok(colored::Color::BrightRed),
        "bright green" | "bright_green" => Ok(colored::Color::BrightGreen),
        "bright yellow" | "bright_yellow" => Ok(colored::Color::BrightYellow),
        "bright blue" | "bright_blue" => Ok(colored::Color::BrightBlue),
        "bright magenta" | "bright_magenta" => Ok(colored::Color::BrightMagenta),
        "bright cyan" | "bright_cyan" => Ok(colored::Color::BrightCyan),
        "bright white" | "bright_white" => Ok(colored::Color::BrightWhite),
        _ => Err(serde::de::Error::custom(format!("Unknown color: {}", s))),
    }
}

impl Prompt {
    pub fn new(prompt: String, color: colored::Color, show_cwd: bool, show_user: bool, show_branch: bool) -> Self {
        Prompt { prompt, color, show_cwd, show_user, show_branch }
    }

    pub fn get_colored_prompt(&self) -> String {
        let mut prompt_parts = Vec::new();

        if self.show_cwd {
            prompt_parts.push(std::env::current_dir().unwrap_or_else(|_| PathBuf::from("")).display().to_string());
        }
        if self.show_user {
            prompt_parts.push(env::var("USER").unwrap_or_else(|_| "".to_string()));
        }
        prompt_parts.push(self.prompt.clone());

        let full_prompt = prompt_parts.join(" ");
        format!("{}", full_prompt.color(self.color))
    }
}
