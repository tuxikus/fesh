use std::env;
use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::logger;

const DEFAULT_CONFIG: &str = r#"
[prompt]
text = "$ "
color = "blue"
show_cwd = true
show_username = false
show_branch = true

[aliases]

[readline]
edit_mode = "emacs"

[env]
"#;

pub struct ConfigParser {
    pub logger: logger::Logger,
    pub config_file: PathBuf,
}

impl ConfigParser {
    pub fn new() -> Self {
        ConfigParser {
            logger: logger::Logger::new(false),
            config_file: Self::get_config_file(),
        }
    }

    fn get_config_file() -> PathBuf {
        if let Ok(path) = env::var("FESH_CONFIG_FILE") {
            return PathBuf::from(path);
        }

        let config_home = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").expect("HOME environment variable not set");
                PathBuf::from(home).join(".config")
            });

        config_home.join("fesh/config.toml")
    }

    pub fn read(&self) -> Result<config::Config, String> {
        let contents = if self.config_file.exists() {
            fs::read_to_string(&self.config_file).map_err(|e| e.to_string())?
        } else {
            eprintln!(
                "info: no config file found at {}",
                self.config_file.display()
            );
            eprintln!("      using default configuration");
            DEFAULT_CONFIG.to_string()
        };

        let config: config::Config = toml::from_str(&contents).map_err(|e| e.to_string())?;
        Ok(config)
    }
}
