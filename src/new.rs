use crate::{license, set_working_dir, workflow};
use anyhow::{ensure, Result};
use std::{fs, path, process};

/// Create a new Rust project with some defaults.
#[derive(clap::Parser)]
pub struct New {
    /// Name of the Rust project.
    name: String,
    #[clap(short = 'p', long)]
    /// Path where the project will be created.
    path: Option<path::PathBuf>,
    /// Full name used in the licenses.
    #[clap(long)]
    full_name: Option<String>,
    /// Create a new Rust library.
    #[clap(short = 'l', long)]
    lib: bool,
    /// Create a new Rust xtask project.
    #[clap(short = 'x', long)]
    xtask: bool,
    /// Do not add licenses.
    #[clap(long)]
    no_license: bool,
    /// Do not add CI.
    #[clap(long)]
    no_ci: bool,
    /// Do not add CI tests for Windows.
    #[clap(long)]
    no_windows: bool,
    /// Do not add CI tests for OSX.
    #[clap(long)]
    no_osx: bool,
}

impl New {
    pub fn run(self) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let project_dir_path = working_dir.join(&self.name);
        fs::create_dir(&project_dir_path)?;

        if self.lib && self.xtask {
            log::info!("Generating project's library package");
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&project_dir_path)
                .args(["new", &self.name, "--lib"]);

            ensure!(
                command.status()?.success(),
                "cannot create project's library package"
            );
        } else if !self.lib && self.xtask {
            log::info!("Generating project's binary package");
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&project_dir_path)
                .args(["new", &self.name]);

            ensure!(
                command.status()?.success(),
                "cannot create project's binary package"
            );
        } else if self.lib && !self.xtask {
            log::info!("Initializing as a library");
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&project_dir_path)
                .args(["init", "--lib"]);

            ensure!(
                command.status()?.success(),
                "cannot initialize as a library"
            );
        } else {
            log::info!("Initializing as a binary");
            let mut command = process::Command::new("cargo");
            command.current_dir(&project_dir_path).arg("init");

            ensure!(command.status()?.success(), "cannot initialize as a binary");
        }

        if !self.no_license {
            log::info!("Generating licenses");
            license::add_licenses(&project_dir_path, self.full_name)?;
        }

        if !self.no_ci {
            let workflows_dir = project_dir_path.join(".github").join("workflows");
            fs::create_dir_all(&workflows_dir)?;

            log::info!("Generating CI");
            if self.lib {
                workflow::add_lib_ci(&workflows_dir, self.no_windows, self.no_osx)?;
            } else {
                workflow::add_bin_ci(&workflows_dir, &self.name, self.no_windows, self.no_osx)?;
            }
        }

        if self.xtask {
            log::info!("Generating xtask package");
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&project_dir_path)
                .args(["new", "xtask"]);

            anyhow::ensure!(
                command.status()?.success(),
                "cannot create project's xtask package"
            );

            log::info!("Generating cargo's config directory");
            let cargo_dir = &project_dir_path.join(".cargo");
            fs::create_dir(cargo_dir)?;

            fs::write(
                cargo_dir.join("config"),
                "[alias]\nxtask = \"run --package xtask --\"",
            )?;

            log::info!("Generating workspace's Cargo.toml");
            fs::write(
                &project_dir_path.join("Cargo.toml"),
                format!(
                    include_str!("../templates/xtask_workspace_toml"),
                    project_name = &self.name
                ),
            )?;
        }

        Ok(())
    }
}
