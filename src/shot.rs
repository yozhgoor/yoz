use std::path::PathBuf;
use anyhow::Result;

/// Take a screenshot using flameshot.
///
/// If nothing is provided as argument, this will open the
/// GUI mode.
#[derive(clap::Parser)]
pub struct Shot {
    /// Show a file for copy using bat.
    #[clap(long)]
    text: Option<PathBuf>,
    /// Capture a single screen.
    #[clap(long)]
    screen: bool,
    /// Capture the entire desktop.
    #[clap(long)]
    desktop: bool,
}

impl Shot {
    pub fn run(self) -> Result<()> {
        if let Some(path) = self.text {
            todo!("bat path --style plain");
        } else if self.screen {
            todo!("flameshot screen");
        } else if self.desktop {
            todo!("flameshot full");
        } else {
            todo!("flameshot gui");
        }
    }
}
