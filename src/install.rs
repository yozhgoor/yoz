use anyhow::Result;
use std::process;
use walkdir::WalkDir;

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
                Manager::Cargo(name).check_or_install()?;
            } else if let Some(url) = self.aur {
                Manager::Aur {
                    program: name,
                    url,
                }.check_or_install()?;
            } else {
                Manager::Pacman(name).check_or_install()?;
            }
        } else {
            if self.prelude {
                let cargo_programs = vec![
                    "alacritty",
                    "bat",
                    "cargo-rdme",
                    "cargo-release",
                    "cargo-temp",
                    "cargo-update",
                    "cargo-watch",
                    "mdbook",
                    "startship",
                ];

                let mut aur_programs = std::collections::HashMap::new();
                aur_programs.insert("spotify", "https://aur.archlinux.org/spotify.git");

                let pacman_programs = vec!["discord", "feh", "flameshot", "neovim"];

                for program in cargo_programs {
                    Manager::Cargo(program.to_string()).check_or_install()?;
                }

                for (program, url) in aur_programs {
                    Manager::Aur { program: program.to_string(), url: url.to_string() }.check_or_install()?;
                }

                for program in pacman_programs {
                    Manager::Pacman(program.to_string()).check_or_install()?;
                }
            } else {
                log::error!("Please select something to install");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Manager {
    Aur {
        program: String,
        url: String,
    },
    Cargo(String),
    Pacman(String),
}

impl Manager {
    fn check_or_install(self) -> Result<()> {
        match self {
            Self::Aur { program, url } => {
                let builds_dir = dirs::home_dir()
                    .expect("cannot get home directory")
                    .join(".builds");

                for entry in WalkDir::new(&builds_dir) {

                }

                let clone_status = process::Command::new("git")
                    .args(["clone", &url])
                    .status()?;
                let install_status = process::Command::new("makepkg")
                    .current_dir(&builds_dir.join(&program))
                    .args(["--syncdeps", "--install", "--clean"])
                    .status()?;
            }
            Self::Cargo(program) => {
                let output = process::Command::new("cargo")
                    .args(["install", "--list"])
                    .output()?;

                let install_status = process::Command::new("cargo")
                    .args(["install", &program])
                    .status()?;
            }
            Self::Pacman(program) => {
                let output = process::Command::new("pacman")
                    .arg("-Q")
                    .output()?;

                let install_status = process::Command::new("sudo")
                    .args(["pacman", "--sync", &program])
                    .status()?;
            }
        }

        Ok(())
    }
}
