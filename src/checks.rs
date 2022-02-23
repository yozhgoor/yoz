use crate::set_working_dir;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::{path, process::Command};

/// Run multiples checks on your Project and output if your code is ok or not.
#[derive(clap::Parser)]
pub struct Checks {
    /// Path of the project that will be checked.
    ///
    /// This path must point to a Rust project.
    path: Option<path::PathBuf>,
    /// Remove the target directory.
    #[clap(long)]
    clean: bool,
}

impl Checks {
    pub fn run(self) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let bar = ProgressBar::new(5);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{bar:25.green} {msg}")
                .progress_chars("#>-"),
        );
        let start = std::time::Instant::now();

        if self.clean {
            bar.set_message("Cleaning...");
            if !Command::new("cargo")
                .current_dir(&working_dir)
                .arg("clean")
                .status()
                .expect("cannot launch `cargo clean`")
                .success()
            {
                log::error!("cannot clean the project");
            }
        }

        bar.set_message("Building...");
        bar.inc(1);

        let (fmt_command, fmt_is_success) = {
            bar.set_message("Checking formatting...");

            let mut command = Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["fmt", "--all", "--check"]);

            let is_success = command.output()?.status.success();

            bar.inc(1);

            (String::from("cargo fmt --all --check"), is_success)
        };

        let (check_command, check_is_success) = {
            bar.set_message("Checking package...");

            let mut command = Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["check", "--workspace", "--all-features"]);

            let is_success = command.output()?.status.success();

            bar.inc(1);

            (
                String::from("cargo check --workspace --all-features"),
                is_success,
            )
        };

        let (test_command, test_is_success) = {
            bar.set_message("Checking tests...");

            let mut command = Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["test", "--workspace", "all-features"]);

            let is_success = command.output()?.status.success();

            bar.inc(1);

            (
                String::from("cargo test --workspace --all-features"),
                is_success,
            )
        };

        let (clippy_command, clippy_is_success) = {
            bar.set_message("Checking lints...");

            let mut command = Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["clippy", "--all", "--tests", "--", "-D", "warnings"]);

            let is_success = command.output()?.status.success();

            bar.inc(1);

            (
                String::from("cargo clippy --all --tests -- -D warnings"),
                is_success,
            )
        };

        bar.finish_with_message(format!("Done (in {}s)", start.elapsed().as_secs()));

        let ok = "Ok".green();
        let nope = "Nope".red();

        let mut failed_command = Vec::new();

        println!();

        if fmt_is_success {
            println!("cargo fmt: {}", ok);
        } else {
            println!("cargo fmt: {}", nope);
            failed_command.push(fmt_command);
        }

        if check_is_success {
            println!("cargo check: {}", ok);
        } else {
            println!("cargo check: {}", nope);
            failed_command.push(check_command);
        }

        if test_is_success {
            println!("cargo test: {}", ok);
        } else {
            println!("cargo test: {}", nope);
            failed_command.push(test_command);
        }

        if clippy_is_success {
            println!("cargo clippy: {}", ok);
        } else {
            println!("cargo clippy: {}", nope);
            failed_command.push(clippy_command);
        }

        println!();

        if !failed_command.is_empty() {
            println!("Failed command:");

            for s in failed_command {
                println!("  {}", s);
            }

            println!();
        }

        Ok(())
    }
}
