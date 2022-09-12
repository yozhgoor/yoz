use anyhow::{bail, ensure, Result};
use std::{path::PathBuf, process};
use walkdir::WalkDir;

/// Update the system.
///
/// This will update archlinux, rust, rust binaries and AUR.
#[derive(Debug, clap::Parser)]
pub struct Update {
    /// Launch all the update commands.
    #[clap(long)]
    all: bool,
    /// Update Arch Linux.
    #[clap(long)]
    arch: bool,
    /// Update all the AUR packages.
    #[clap(long)]
    aur: bool,
    /// Update Rust.
    #[clap(long)]
    rust: bool,
    /// Update all the rust binaries installed.
    #[clap(long)]
    rust_bin: bool,
}

impl Update {
    pub fn run(self, aur_dir: Option<PathBuf>) -> Result<()> {
        if self.arch || self.all {
            let mut update = process::Command::new("sudo");
            update.args(["pacman", "--sync", "--refresh", "--sysupgrade", "--clean"]);

            ensure!(update.status()?.success(), "cannot update ArchLinux");

            let mut cache = process::Command::new("sudo");
            update.args(["pacman", "--sync", "--clean"]);

            ensure!(
                cache.status()?.success(),
                "cannot clean pacman cache directory"
            );

            let mut orphans = process::Command::new("sudo");
            orphans.args([
                "pacman",
                "--remove",
                "--nosave",
                "--recursive",
                "$(pacman --query --unrequired --deps --quiet)",
            ]);

            ensure!(cache.status()?.success(), "cannot clean orphans packages");
        }

        if self.aur || self.all {
            let dir = if let Some(dir) = aur_dir {
                dir
            } else {
                bail!("Please configure `aur_directory` in your config file")
            };

            for entry in WalkDir::new(&dir)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                let mut pull_command = process::Command::new("git");
                pull_command.current_dir(path).arg("pull");

                let mut make_command = process::Command::new("makepkg");
                make_command
                    .current_dir(path)
                    .args(["--syncdeps", "--install", "--clean"]);

                ensure!(
                    pull_command.status()?.success(),
                    "cannot pull source at {}",
                    path.display()
                );
                ensure!(
                    make_command.status()?.success(),
                    "cannot make package at {}",
                    path.display()
                );
            }
        } else if self.rust || self.all {
            let mut command = process::Command::new("rustup");
            command.arg("update");

            ensure!(command.status()?.success(), "cannot update Rust");
        } else if self.rust_bin || self.all {
            let mut command = process::Command::new("cargo");
            command.args(["install-update", "--all"]);

            ensure!(command.status()?.success(), "cannot update Rust binaries");
        } else {
            log::error!("Please give a specific flag or pass --all")
        }

        Ok(())
    }
}
