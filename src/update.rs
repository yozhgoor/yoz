use anyhow::Result;
use std::process;
use walkdir::WalkDir;

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
    pub fn run(self) -> Result<()> {
        let mut commands = self.generate_commands()?;

        for command in commands.iter_mut() {
            if command.status()?.success() {
                log::info!("Success");
            } else {
                log::error!("an error occurred");
            }
        }

        Ok(())
    }

    pub fn generate_commands(&self) -> Result<Vec<process::Command>> {
        let mut commands = Vec::new();

        if self.arch || self.all {
            let mut command = process::Command::new("sudo");
            command.args(["pacman", "--sync", "--refresh", "--sysupgrade", "--clean"]);

            commands.push(command);
        }

        if self.aur || self.all {
            let home_dir = dirs::home_dir().expect("cannot get home directory");

            for entry in WalkDir::new(home_dir.join(".builds"))
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let mut pull_command = process::Command::new("git");
                pull_command.current_dir(entry.path()).arg("pull");

                let mut make_command = process::Command::new("makepkg");
                make_command
                    .current_dir(entry.path())
                    .args(["--syncdeps", "--install", "--clean"]);
                pull_command.status()?;

                make_command.status()?;
            }
        }

        if self.rust || self.all {
            let mut command = process::Command::new("rustup");
            command.arg("update");

            commands.push(command);
        }

        if self.rust_bin || self.all {
            let mut command = process::Command::new("cargo");
            command.args(["install-update", "--all"]);

            commands.push(command);
        }

        if commands.is_empty() {
            log::error!("Please select something to update, or pass `--all`");
        }

        Ok(commands)
    }
}
