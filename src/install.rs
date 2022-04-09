use anyhow::{bail, ensure, Result};
use std::{process, path::PathBuf};

/// Install a package.
///
/// If no flags are provided, this command will
/// use pacman.
#[derive(Debug, clap::Parser)]
pub struct Install {
    /// Name of the package to install.
    name: String,
    /// Install the package via cargo.
    #[clap(long, short = 'c')]
    cargo: bool,
    /// Install the package via AUR.
    #[clap(long, short = 'a')]
    aur: Option<String>,
}

impl Install {
    pub fn run(self, aur_dir: Option<PathBuf>) -> Result<()> {
        match process::Command::new(&self.name)
            .arg("--version")
            .status() {
                Ok(status) if status.success() => {
                    log::info!("{} is already installed", &self.name)
                }
                _ => {
                    if self.cargo {
                        Manager::Cargo(self.name).install()?;
                    } else if let Some(url) = self.aur {
                        Manager::Aur { program: self.name, url, dir: aur_dir }.install()?;
                    } else {
                        Manager::Pacman(self.name).install()?;
                    }
                }
            }

        Ok(())
    }
}

#[derive(Debug)]
enum Manager {
    Aur { program: String, url: String, dir: Option<PathBuf> },
    Cargo(String),
    Pacman(String),
}

impl Manager {
    fn install(self) -> Result<()> {
        match self {
            Self::Aur { program, url, dir } => {
                let dir = if let Some(dir) = dir {
                    dir
                } else {
                    bail!("Please configure `aur_directory` in your config file")
                };

                ensure!(
                    process::Command::new("git")
                        .current_dir(&dir)
                        .args(["clone", &url])
                        .status()?
                        .success(),
                    "cannot clone source from {}", &url
                );

                ensure!(
                    process::Command::new("makepkg")
                        .current_dir(&dir.join(&program))
                        .args(["--syncdeps", "--install", "--clean"])
                        .status()?
                        .success(),
                    "cannot install {} from AUR", &program
                );
            }
            Self::Cargo(program) => {
                ensure!(
                    process::Command::new("cargo")
                        .args(["install", &program])
                        .status()?
                        .success(),
                    "cannot install {} via cargo", &program
                );
            }
            Self::Pacman(program) => {
                ensure!(
                    process::Command::new("sudo")
                        .args(["pacman", "--sync", &program])
                        .status()?
                        .success(),
                    "cannot install {} via pacman", &program
                );
            }
        }

        Ok(())
    }
}
