use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api_token: String,
    pub hotkey: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_url: "http://localhost:8000".to_string(),
            api_token: "changeme".to_string(),
            hotkey: "super+c".to_string(),
            language: "en".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("voice-type");
        config_dir.join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();

        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            // Create default config
            let config = Config::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();

        // Create directory if needed
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(content) = toml::to_string_pretty(self) {
            let _ = fs::write(&path, content);
        }
    }
}
