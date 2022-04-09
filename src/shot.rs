use anyhow::{ensure, Result};
use std::{path::PathBuf, process};

/// Take a screenshot using flameshot.
///
/// If nothing is provided as argument, this will open the
/// GUI mode.
#[derive(Debug, clap::Parser)]
pub struct Shot {
    /// Show a file for copy using bat.
    #[clap(long)]
    text: Option<PathBuf>,
    /// Capture a single screen.
    #[clap(long)]
    screen: Option<bool>,
    /// Capture the entire desktop.
    #[clap(long)]
    desktop: Option<bool>,
}

impl Shot {
    pub fn run(self) -> Result<()> {
        if let Some(path) = self.text {
            ensure!(
                process::Command::new("bat")
                    .arg(&path)
                    .arg("--style")
                    .arg("plain")
                    .status()?
                    .success(),
                "cannot open {} with bat",
                path.display()
            );
        } else if let Some(clipboard) = self.screen {
            let mut process = process::Command::new("flameshot");
            process.arg("screen");

            if clipboard {
                process.arg("--clipboard");
            }

            ensure!(process.status()?.success(), "`flameshot screen` failed");
        } else if let Some(clipboard) = self.desktop {
            let mut process = process::Command::new("flameshot");
            process.arg("full");

            if clipboard {
                process.arg("--clipboard");
            }

            ensure!(process.status()?.success(), "`flameshot full` failed");
        } else {
            ensure!(
                process::Command::new("flameshot")
                    .arg("gui")
                    .status()?
                    .success(),
                "`flameshot gui` failed"
            );
        }

        Ok(())
    }
}
