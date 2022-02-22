use crate::set_working_dir;
use anyhow::Result;
use std::{path, process};
use colored::*;
use indicatif::ProgressBar;

use std::fmt::Write;

/// Run multiples checks on your Project and output if your code is ok or not.
#[derive(clap::Parser)]
pub struct Checks {
    /// Path of the project that will be checked.
    ///
    /// This path must point to a Rust project.
    path: Option<path::PathBuf>,
    /// Arguments given to the `cargo check` command.
    ///
    /// The default is `cargo check --workspace --all-features`.
    #[clap(long)]
    check: Vec<String>,
    /// Arguments given to the `cargo fmt` command.
    ///
    /// The default is `cargo fmt --all --check`.
    #[clap(long)]
    fmt: Vec<String>,
    /// Arguments given to the `cargo test` command.
    ///
    /// The default is `cargo test --workspace --all-features`.
    #[clap(long)]
    test: Vec<String>,
    /// Arguments given to the `cargo clippy` command.
    ///
    /// The default is `cargo clippy --all --tests -- -D warnings`.
    #[clap(long)]
    clippy: Vec<String>,
    /// Remove the target directory.
    #[clap(long)]
    clean: bool,
}

impl Checks {
    pub fn run(self) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        if self.clean {
            if process::Command::new("cargo")
                .current_dir(&working_dir)
                .arg("clean")
                .status()
                .expect("cannot launch `cargo clean`")
                .success()
            {
                log::info!("Cleaned");
            } else {
                log::error!("command `cargo clean` failed");
            }
        }

        println!();

        let mut check = if !self.check.is_empty() {
            let mut command = process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.check);

            command
        } else {
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["check", "--workspace", "--all-features"]);

            command
        };

        let mut test = if !self.test.is_empty() {
            let mut command = process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.test);

            command
        } else {
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["test", "--workspace", "--all-features"]);

            command
        };

        let mut fmt = if !self.fmt.is_empty() {
            let mut command = process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.fmt);

            command
        } else {
            let mut command = process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["fmt", "--all", "--check"]);

            command
        };

        let mut clippy = if !self.clippy.is_empty() {
            let mut command = process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.clippy);

            command
        } else {

                let mut command = process::Command::new("cargo");
                command
                    .current_dir(&working_dir)
                    .args(["clippy", "--all", "--tests", "--", "-D", "warnings"]);

                command
        };

        let mut output = String::new();

        let bar = ProgressBar::new_spinner();

        let ok = "Ok".green();
        let nope = "Nope".red();

        if check.output()?.status.success() {
            writeln!(output, "cargo check : {}", ok)?;
        } else {
            writeln!(output, "cargo check : {}", nope)?;
        };

        if test.output()?.status.success() {
            writeln!(output, "cargo test  : {}", ok)?;
        } else {
            writeln!(output, "cargo test  : {}", nope)?;
        };

        if fmt.output()?.status.success() {
            writeln!(output, "cargo fmt   : {}", ok)?;
        } else {
            writeln!(output, "cargo fmt   : {}", nope)?;
        };

        if clippy.output()?.status.success() {
            writeln!(output, "cargo clippy: {}", ok)?;
        } else {
            writeln!(output, "cargo clippy: {}", nope)?;
        };

        bar.finish_and_clear();

        println!("{}", output);

        Ok(())
    }
}
