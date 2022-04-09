use crate::{background::Position, screen::Monitor};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // checks
    #[serde(default, rename = "check_args", skip_serializing_if = "Vec::is_empty")]
    pub default_check_args: Vec<String>,
    #[serde(default, rename = "test_args", skip_serializing_if = "Vec::is_empty")]
    pub default_test_args: Vec<String>,
    #[serde(default, rename = "fmt_args", skip_serializing_if = "Vec::is_empty")]
    pub default_fmt_args: Vec<String>,
    #[serde(default, rename = "clippy_args", skip_serializing_if = "Vec::is_empty")]
    pub default_clippy_args: Vec<String>,
    // launch
    #[serde(
        default,
        rename = "launch_command",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_launch_command: Vec<String>,
    #[serde(
        default,
        rename = "terminal_command",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_terminal_command: Vec<String>,
    // licenses
    #[serde(rename = "full_name")]
    pub default_full_name: Option<String>,
    // screen
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
    // background
    #[serde(rename = "background_file_path")]
    pub default_bg_file_path: Option<PathBuf>,
    #[serde(rename = "background_position")]
    pub default_bg_position: Option<Position>,
    // install/update
    #[serde(rename = "aur_directory")]
    pub aur_dir: Option<PathBuf>,
}

impl Config {
    fn new() -> Self {
        Self {
            default_check_args: Vec::new(),
            default_test_args: Vec::new(),
            default_fmt_args: Vec::new(),
            default_clippy_args: Vec::new(),
            default_launch_command: Vec::new(),
            default_terminal_command: Vec::new(),
            default_full_name: None,
            default_bg_file_path: None,
            default_bg_position: None,
            main_monitor: None,
            external_monitor: None,
            aur_dir: None,
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
