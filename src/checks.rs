use crate::set_working_dir;
use anyhow::Result;
use colored::*;
use std::{path, process::Command};
use indicatif::{ProgressBar, ProgressStyle};

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
    #[clap(long = "check")]
    check_args: Vec<String>,
    /// Arguments given to the `cargo fmt` command.
    ///
    /// The default is `cargo fmt --all --check`.
    #[clap(long = "fmt")]
    fmt_args: Vec<String>,
    /// Arguments given to the `cargo test` command.
    ///
    /// The default is `cargo test --workspace --all-features`.
    #[clap(long = "test")]
    test_args: Vec<String>,
    /// Arguments given to the `cargo clippy` command.
    ///
    /// The default is `cargo clippy --all --tests -- -D warnings`.
    #[clap(long = "clippy")]
    clippy_args: Vec<String>,
    /// Remove the target directory.
    #[clap(long)]
    clean: bool,
}

impl Checks {
    pub fn run(self) -> Result<()> {
        let checks = self.execute()?;

        checks.print()?;

        Ok(())
    }

    fn execute(self) -> Result<ExecutedChecks> {
        let working_dir = set_working_dir(self.path)?;

        let bar = ProgressBar::new(5);
        bar.set_style(ProgressStyle::default_bar().template("{bar:25.green} {msg}").progress_chars("#>-"));
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

        let check = {
            bar.set_message("Checking package...");

            let mut command_string = String::from("cargo check");

            let mut command = Command::new("cargo");
            command.current_dir(&working_dir).arg("check");

            if self.check_args.is_empty() {
                command.args(["--workspace", "--all-features"]);

                command_string.push_str(" --workspace --all-features");
            } else {
                command.args(&self.check_args);

                for arg in self.check_args {
                    command_string.push_str(format!(" {}", arg).as_str());
                }


            }

            let is_success = command.output()?.status.success();

            bar.inc(1);

            ExecutedCheck {
                command_string,
                is_success
            }
        };

        let test = {
            bar.set_message("Checking tests...");
            let mut command_string = String::from("cargo test");

            let mut command = Command::new("cargo");
            command.current_dir(&working_dir).arg("test");


            if self.test_args.is_empty() {
                command.args(["--workspace", "all-features"]);

                command_string.push_str(" --workspace --all-features");
            } else {
                command.args(&self.test_args);

                for arg in self.test_args {
                    command_string.push_str(format!(" {}", arg).as_str());
                }
            }

            let is_success = command.output()?.status.success();
            bar.inc(1);

            ExecutedCheck {
                command_string,
                is_success
            }
        };

        let fmt = {
            bar.set_message("Checking formatting...");

            let mut command_string = String::from("cargo fmt");

            let mut command = Command::new("cargo");
            command.current_dir(&working_dir).arg("fmt");

            if self.fmt_args.is_empty() {
                command.args(["--all", "--check"]);

                command_string.push_str(" --all --check")
            } else {
                command.args(&self.fmt_args);

                for arg in self.fmt_args {
                    command_string.push_str(format!(" {}", arg).as_str());
                }
            }

            let is_success = command.output()?.status.success();
            bar.inc(1);

            ExecutedCheck {
                command_string,
                is_success
            }
        };

        let clippy = {
            bar.set_message("Checking lints...");

            let mut command_string = String::from("cargo clippy");

            let  mut command = Command::new("cargo");
            command.current_dir(&working_dir).arg("clippy");

            if self.clippy_args.is_empty() {
                command.args(["--all", "--tests", "--", "-D", "warnings"]);

                command_string.push_str(" --all --tests -- -D warnings");
            } else {
                command.args(&self.clippy_args);

                for arg in self.clippy_args {
                    command_string.push_str(format!(" {}", arg).as_str());
                }
            }

            let is_success = command.output()?.status.success();

            bar.inc(1);

            ExecutedCheck {
                command_string,
                is_success,
            }
        };

        bar.finish_with_message(format!("Done (in {}s)", start.elapsed().as_secs()));

        Ok(ExecutedChecks {
            check,
            test,
            fmt,
            clippy,
        })
    }
}

struct ExecutedChecks {
    check: ExecutedCheck,
    test: ExecutedCheck,
    fmt: ExecutedCheck,
    clippy: ExecutedCheck,
}

impl ExecutedChecks {
    pub fn print(self) -> Result<()> {
        let ok = "Ok".green();
        let nope = "Nope".red();

        let mut failed_command = Vec::new();

        println!();

        if self.check.is_success {
            println!("cargo check: {}", ok);
        } else {
            println!("cargo check: {}", nope);
            failed_command.push(self.check.command_string);
        }

        if self.test.is_success {
            println!("cargo test: {}", ok);
        } else {
            println!("cargo test: {}", nope);
            failed_command.push(self.test.command_string);
        }

        if self.fmt.is_success {
            println!("cargo fmt: {}", ok);
        } else {
            println!("cargo fmt: {}", nope);
            failed_command.push(self.fmt.command_string);
        }

        if self.clippy.is_success {
            println!("cargo clippy: {}", ok);
        } else {
            println!("cargo clippy: {}", nope);
            failed_command.push(self.clippy.command_string);
        }

        println!();
        println!("Failed command");

        for s in failed_command {
            println!("  {}", s);
        }

        Ok(())
    }
}

struct ExecutedCheck {
    command_string: String,
    is_success: bool,
}
