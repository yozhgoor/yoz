use crate::{background::Position, screen::Monitor};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Launch
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_launch_command: Vec<String>,
    // Licenses
    pub default_full_name: Option<String>,
    // Screen
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
    // Background
    pub bg_file_path: Option<PathBuf>,
    pub bg_position: Option<Position>,
}

impl Config {
    fn new() -> Self {
        Self {
            default_launch_command: Vec::new(),
            default_full_name: None,
            main_monitor: None,
            external_monitor: None,
            bg_file_path: None,
            bg_position: None,
        }
    }

    pub fn get_or_create() -> Result<Self> {
        let config_file_path =
            xdg::BaseDirectories::with_prefix("yoz")?.place_config_file("config.toml")?;

        let config: Self = match fs::read(&config_file_path) {
            Ok(file) => toml::de::from_slice(&file)?,
            Err(_) => {
                let config = Self::new();
                fs::write(&config_file_path, toml::ser::to_string(&config)?)?;
                println!("Config file created at: {}", config_file_path.display());

                config
            }
        };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
