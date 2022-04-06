use crate::screen::Monitor;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    // Licenses
    pub default_full_name: Option<String>,
    // Screen
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
    pub bg_file_path: Option<PathBuf>,
    pub bg_position: Option<Position>,
}

impl Config {
    fn new() -> Self {
        Self {
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

#[derive(Debug, Deserialize, Serialize)]
pub enum Position {
    Center,
    Fill,
    Max,
    Scale,
    Tile,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Center => write!(f, "center"),
            Self::Fill => write!(f, "fill"),
            Self::Max => write!(f, "max"),
            Self::Scale => write!(f, "scale"),
            Self::Tile => write!(f, "tile"),
        }
    }
}
