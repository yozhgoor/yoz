use crate::{background::Position, screen::Monitor};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, rename = "check_args", skip_serializing_if = "Vec::is_empty")]
    pub default_check_args: Vec<String>,
    #[serde(default, rename = "test_args", skip_serializing_if = "Vec::is_empty")]
    pub default_test_args: Vec<String>,
    #[serde(default, rename = "fmt_args", skip_serializing_if = "Vec::is_empty")]
    pub default_fmt_args: Vec<String>,
    #[serde(default, rename = "clippy_args", skip_serializing_if = "Vec::is_empty")]
    pub default_clippy_args: Vec<String>,
    #[serde(rename = "full_name")]
    pub default_full_name: Option<String>,
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
    #[serde(rename = "background_file_path")]
    pub default_bg_file_path: Option<PathBuf>,
    #[serde(rename = "background_position")]
    pub default_bg_position: Option<Position>,
    #[serde(rename = "aur_directory")]
    pub aur_dir: Option<PathBuf>,
    pub temporary_project_path: Option<PathBuf>,
    pub default_editor: Option<PathBuf>,
    pub default_terminal: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subprocess_command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fonts: Vec<String>,
    #[serde(default)]
    pub fonts_size: u32,
    #[serde(rename = "browser")]
    pub default_browser: Option<String>,
    pub bar_path: Option<PathBuf>,
    pub bar_position: Option<String>,
    pub home_symbol: Option<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            default_check_args: Vec::new(),
            default_test_args: Vec::new(),
            default_fmt_args: Vec::new(),
            default_clippy_args: Vec::new(),
            default_full_name: None,
            default_bg_file_path: None,
            default_bg_position: None,
            main_monitor: None,
            external_monitor: None,
            aur_dir: None,
            temporary_project_path: None,
            default_editor: None,
            default_terminal: None,
            subprocess_command: Vec::new(),
            fonts: Vec::new(),
            fonts_size: 0,
            default_browser: None,
            bar_path: None,
            bar_position: None,
            home_symbol: None,
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

    fn dot() -> Self {
        Self {
            default_check_args: vec!["--workspace".to_string(), "--all-features".to_string()],
            default_test_args: vec!["--workspace".to_string(), "--all-features".to_string()],
            default_fmt_args: vec!["--all".to_string(), "--check".to_string()],
            default_clippy_args: vec![
                "--all".to_string(),
                "--all-features".to_string(),
                "--tests".to_string(),
                "--".to_string(),
                "-D".to_string(),
                "warnings".to_string(),
            ],
            default_full_name: Some("Yohan Boogaert".to_string()),
            default_bg_file_path: Some(PathBuf::from("/home/yozhgoor/Pictures/BG_1920_1080.png")),
            default_bg_position: Some(Position::Fill),
            main_monitor: Some(Monitor {
                name: "eDP-1".to_string(),
                width: 1920,
                height: 1080,
                rate: 60,
            }),
            external_monitor: Some(Monitor {
                name: "HDMI-1".to_string(),
                width: 1920,
                height: 1080,
                rate: 144,
            }),
            aur_dir: Some(PathBuf::from("/home/yozhgoor/.builds")),
            temporary_project_path: Some(PathBuf::from("/home/yozhgoor/.cache/cargo-temp/")),
            default_editor: Some(PathBuf::from("/usr/bin/nvim")),
            default_terminal: Some(PathBuf::from("/usr/bin/alacritty")),
            subprocess_command: vec![
                "alacritty".to_string(),
                "-e".to_string(),
                "cargo".to_string(),
                "watch".to_string(),
                "-x".to_string(),
                "check".to_string(),
            ],
            fonts: vec![
                "Hack Nerd Font".to_string(),
                "DejaVu Sans Mono".to_string(),
                "Font Awesome".to_string(),
            ],
            fonts_size: 8,
            default_browser: Some("Firefox".to_string()),
            bar_path: Some(PathBuf::from("/home/yozhgoor/.config/i3status-rs/config")),
            bar_position: Some("top".to_string()),
            home_symbol: Some("yoz".to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
