use anyhow::Result;

/// Generate config files from dotfiles
#[derive(Debug, clap::Parser)]
pub struct Dotfiles {}

impl Dotfiles {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
