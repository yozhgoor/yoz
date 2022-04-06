use crate::config::Position;
use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf, process, str::FromStr};

/// Set up screens using xrandr.
///
/// If nothing is provided as argument, this will list the detected monitors.
///
/// This subcommand aims to be used with a laptop and, optionally, an external
/// screen.
#[derive(clap::Parser)]
pub struct Screen {
    /// Enable the main screen and disable the external screen.
    #[clap(long)]
    main: bool,
    /// Enable the external screen and disable the main screen.
    #[clap(long)]
    external: bool,
    /// Set the refresh rate of the external monitor
    #[clap(long, default_value_t = 60)]
    rate: u8,
    /// Set the position of the external screen related to the position of the
    /// laptop screen.
    #[clap(long, default_value_t = Direction::Right)]
    direction: Direction,
}

impl Screen {
    pub fn run(
        self,
        main_monitor: Option<Monitor>,
        external_monitor: Option<Monitor>,
        bg_file_path: Option<PathBuf>,
        bg_position: Option<Position>,
    ) -> Result<()> {
        let main_monitor = match main_monitor {
            Some(monitor) => monitor,
            None => bail!("main monitor not configured"),
        };
        let main_monitor_mode = format!("{}x{}", main_monitor.width, main_monitor.height);

        let external_monitor = match external_monitor {
            Some(monitor) => monitor,
            None => bail!("external monitor not configured"),
        };
        let external_monitor_mode =
            format!("{}x{}", external_monitor.width, external_monitor.height);

        if self.main {
            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(main_monitor.name)
                    .arg("--mode")
                    .arg(main_monitor_mode)
                    .arg("--refresh")
                    .arg(format!("{}", main_monitor.rate))
                    .status()?
                    .success(),
                "cannot enable main monitor",
            );

            if !process::Command::new("xrandr")
                .arg("--output")
                .arg(external_monitor.name)
                .arg("--off")
                .status()?
                .success()
            {
                log::error!("cannot disable external monitor");
            }

            if let Some(file_path) = bg_file_path {
                let mut process = process::Command::new("feh");
                if let Some(position) = bg_position {
                    process.arg(format!("--bg-{}", position));
                } else {
                    process.arg("--bg-fill");
                }

                process.arg(file_path);

                if !process.status()?.success() {
                    log::error!("cannot set the desktop background")
                }
            }
        } else if self.external {
            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(external_monitor.name)
                    .arg("--mode")
                    .arg(external_monitor_mode)
                    .arg("--refresh")
                    .arg(format!("{}", external_monitor.rate))
                    .status()?
                    .success(),
                "cannot enable external monitor",
            );

            if !process::Command::new("xrandr")
                .arg("--output")
                .arg(main_monitor.name)
                .arg("--off")
                .status()?
                .success()
            {
                log::error!("cannot disable main monitor");
            }

            if let Some(file_path) = bg_file_path {
                let mut process = process::Command::new("feh");
                if let Some(position) = bg_position {
                    process.arg(format!("--bg-{}", position));
                } else {
                    process.arg("--bg-fill");
                }

                process.arg(file_path);

                if !process.status()?.success() {
                    log::error!("cannot set the desktop background")
                }
            }
        } else {
            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(&external_monitor.name)
                    .arg("--mode")
                    .arg(external_monitor_mode)
                    .arg("--refresh")
                    .arg(format!("{}", external_monitor.rate))
                    .status()?
                    .success(),
                "cannot enable external monitor"
            );

            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(main_monitor.name)
                    .arg(format!("--{}-of", self.direction))
                    .arg(external_monitor.name)
                    .arg("--mode")
                    .arg(main_monitor_mode)
                    .arg("--refresh")
                    .arg(format!("{}", main_monitor.rate))
                    .status()?
                    .success(),
                "cannot enable main monitor",
            );

            if let Some(file_path) = bg_file_path {
                let mut process = process::Command::new("feh");
                if let Some(position) = bg_position {
                    process.arg(format!("--bg-{}", position));
                } else {
                    process.arg("--bg-fill");
                }

                process.arg(file_path);

                if !process.status()?.success() {
                    log::error!("cannot set the desktop background")
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub rate: u8,
}

enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.to_lowercase();

        let direction = match s.as_str() {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => bail!("Cannot parse direction from {}", s),
        };

        Ok(direction)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}
