use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub file_dir: Option<String>,
    pub library_name: Option<String>,
    pub library_path: Option<String>,
}

impl Config {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("cp_unfold").join("config.toml"))
    }

    pub fn exists() -> bool {
        Self::config_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    pub fn load() -> Self {
        let config_path = match Self::config_path() {
            Some(path) => path,
            None => return Config::default(),
        };

        if !config_path.exists() {
            return Config::default();
        }

        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(_) => return Config::default(),
        };

        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()
            .ok_or("Could not determine config directory")?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }
}
