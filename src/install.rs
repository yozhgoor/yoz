use anyhow::Result;
use std::process;

#[derive(Debug, clap::Parser)]
pub struct Install {
    /// Name of the package to install.
    name: Option<String>,
    /// Install the package via cargo.
    #[clap(long, short = 'c')]
    cargo: bool,
    /// Install the package via AUR.
    #[clap(long, short = 'a')]
    aur: Option<String>,
    /// Install the prelude of a yoz system.
    #[clap(long)]
    prelude: bool,
}

impl Install {
    pub fn run(self) -> Result<()> {
        if let Some(name) = self.name {
            if self.cargo {
                process::Command::new("cargo")
                    .args(["install", &name])
                    .status()?;
            } else if let Some(url) = self.aur {
                let builds_dir = dirs::home_dir().expect("cannot get home directory");
                process::Command::new("git")
                    .current_dir(&builds_dir)
                    .args(["clone", &url])
                    .status()?;
                process::Command::new("makepkg")
                    .current_dir(&builds_dir.join(&name))
                    .args(["--syncdeps", "--install", "--clean"]);
            } else {
                process::Command::new("sudo")
                    .args(["pacman", "--sync", &name])
                    .status()?;
            }
        } else {
            if self.prelude {
                unimplemented!();
            } else {
                log::error!("Please select something to install");
            }
        }

        Ok(())
    }
}
