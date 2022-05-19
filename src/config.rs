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
    #[serde(rename = "background_file_path")]
    pub default_bg_file_path: Option<PathBuf>,
    #[serde(rename = "background_position")]
    pub default_bg_position: Option<Position>,
    #[serde(rename = "aur_directory")]
    pub aur_dir: Option<PathBuf>,
    pub temporary_project_path: Option<PathBuf>,
    pub default_editor: Option<String>,
    pub default_terminal: Option<String>,
    #[serde(default, rename = "fonts", skip_serializing_if = "Vec::is_empty")]
    pub default_fonts: Vec<String>,
    #[serde(rename = "fonts_size")]
    pub default_fonts_size: Option<u32>,
    #[serde(rename = "browser")]
    pub default_browser: Option<String>,
    #[serde(rename = "home_symbol")]
    pub default_home_symbol: Option<String>,
    #[serde(rename = "net_device")]
    pub default_net_device: Option<String>,
    pub main_monitor: Option<Monitor>,
    pub external_monitor: Option<Monitor>,
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
            aur_dir: None,
            temporary_project_path: None,
            default_editor: None,
            default_terminal: None,
            default_fonts: Vec::new(),
            default_fonts_size: None,
            default_browser: None,
            default_home_symbol: None,
            default_net_device: None,
            main_monitor: None,
            external_monitor: None,
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

            aur_dir: Some(PathBuf::from("/home/yozhgoor/.builds")),
            temporary_project_path: Some(PathBuf::from("/home/yozhgoor/.cache/cargo-temp/")),
            default_editor: Some("nvim".to_string()),
            default_terminal: Some("alacritty".to_string()),
            default_fonts: vec![
                "Hack Nerd Font".to_string(),
                "DejaVu Sans Mono".to_string(),
                "Font Awesome".to_string(),
            ],
            default_fonts_size: Some(8),
            default_browser: Some("Firefox".to_string()),
            default_home_symbol: Some("yoz".to_string()),
            default_net_device: Some("wlp1s0".to_string()),
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
        }
    }

    pub fn create_from_dot() -> Result<()> {
        let config_file_path =
            xdg::BaseDirectories::with_prefix("yoz")?.place_config_file("config.toml")?;

        let config = Self::dot();
        fs::write(&config_file_path, toml::ser::to_string(&config)?)?;
        println!("Config file created at: {}", config_file_path.display());

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
