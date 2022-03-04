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
                Manager::Cargo.check_or_install(name, None)?;
            } else if self.aur.is_some() {
                Manager::Aur.check_or_install(name, self.aur)?;
            } else {
                Manager::Pacman.check_or_install(name, None)?;
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

                let pacman_programs = vec![
                    "discord",
                    "feh",
                    "flameshot",
                    "neovim",
                ];

                for program in cargo_programs {
                    Manager::Cargo.check_or_install(program.to_string(), None)?;
                }

                for (program, url) in aur_programs {
                    Manager::Aur.check_or_install(program.to_string(), Some(url.to_string()))?;
                }

                for program in pacman_programs {
                    Manager::Pacman.check_or_install(program.to_string(), None)?;
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
    Aur,
    Cargo,
    Pacman,
}

impl Manager {
    fn check_or_install(self, program: String, url: Option<String>) -> Result<()> {
        let check_status = process::Command::new(&program).arg("--version").status();

        if let Err(_) = check_status {
            log::info!("Installing {}", &program);
            match self {
                Self::Aur => {
                    let builds_dir = dirs::home_dir().expect("cannot get home directory");
                    let url = url.expect("Cannot install via AUR without url");
                    process::Command::new("git")
                        .current_dir(&builds_dir)
                        .args(["clone", &url])
                        .status()?;
                    process::Command::new("makepkg")
                        .current_dir(&builds_dir.join(&program))
                        .args(["--syncdeps", "--install", "--clean"]);
                }
                Self::Cargo => {
                    process::Command::new("cargo")
                        .args(["install", &program])
                        .status()?;
                }
                Self::Pacman => {
                    process::Command::new("sudo")
                        .args(["pacman", "--sync", &program])
                        .status()?;
                }
            }
        } else if check_status?.success() {
            log::info!("{} is already installed", &program);
        } else {
            log::error!("an error occurred when checking {}", &program);
        }

        Ok(())
    }
}
