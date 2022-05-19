use crate::value_or_default;
use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf, process, str::FromStr};

/// Set the background.
///
/// You can optionally use a path to an image and a position.
///
/// If no image are provided, this will fail.
#[derive(Debug, clap::Parser)]
pub struct Background {
    /// Path of the image to set as background.
    #[clap(long, short = 'f')]
    file_path: Option<PathBuf>,
    /// Position of the background image
    ///
    /// Available options are: Center, Fill, Max, Scale, Tile.
    #[clap(long, short = 'p')]
    position: Option<Position>,
}

impl Background {
    pub fn run(self, bg_file_path: Option<PathBuf>, bg_position: Option<Position>) -> Result<()> {
        let bg_file_path = value_or_default(self.file_path, bg_file_path, "bg_file_path")?;
        let bg_position = value_or_default(self.position, bg_position, "bg_position")?;

        let mut process = process::Command::new("feh");
        process.arg("--no-fehbg");
        process.arg(format!("--bg-{}", bg_position));
        process.arg(bg_file_path);

        ensure!(process.status()?.success(), "cannot set the background");

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Position {
    Center,
    Fill,
    Max,
    Scale,
    Tile,
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.to_lowercase();

        let position = match s.as_str() {
            "center" => Self::Center,
            "fill" => Self::Fill,
            "max" => Self::Max,
            "scale" => Self::Scale,
            "tile" => Self::Tile,
            _ => bail!("Cannot parse position from {}", s),
        };

        Ok(position)
    }
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
