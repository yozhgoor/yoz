use crate::screen::Monitor;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            main_monitor: None,
            external_monitor: None,
        }
    }

    pub fn get_or_create() -> Result<Self> {
        #[cfg(unix)]
        let config_file_path =
            { xdg::BaseDirectories::with_prefix("yoz")?.place_config_file("config.toml")? };
        #[cfg(windows)]
        let config_file_path = {
            dirs::config_dir()
                .context("could not get config directory")?
                .join(env!("CARGO_PKG_NAME"));
            let _ = fs::create_dir_all(&config_dir);

            config_dir.join("config.toml")
        };

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
