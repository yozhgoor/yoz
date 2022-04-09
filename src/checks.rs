use crate::set_working_dir;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{path, process, time};

/// Run multiples checks on your project.
#[derive(Debug, clap::Parser)]
pub struct Checks {
    /// Path of the project that will be checked.
    ///
    /// This path must point to a Rust project.
    path: Option<path::PathBuf>,
    /// Remove the target directory.
    #[clap(long)]
    clean: bool,
    /// Arguments given to the `cargo check` command.
    #[clap(long = "check")]
    check_args: Vec<String>,
    /// Arguments given to the `cargo test` command.
    #[clap(long = "test")]
    test_args: Vec<String>,
    /// Arguments given to the `cargo fmt` command.
    #[clap(long = "fmt")]
    fmt_args: Vec<String>,
    /// Arguments given to the `cargo clippy` command.
    #[clap(long = "clippy")]
    clippy_args: Vec<String>,
}

impl Checks {
    pub fn run(
        self,
        default_check_args: Vec<String>,
        default_test_args: Vec<String>,
        default_fmt_args: Vec<String>,
        default_clippy_args: Vec<String>,
    ) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let start = std::time::Instant::now();

        if self.clean {
            if process::Command::new("cargo")
                .current_dir(&working_dir)
                .arg("clean")
                .output()?
                .status
                .success()
            {
                log::info!("Cleaned");
            } else {
                log::error!("`cargo clean` failed");
            }
        }

        let check_args = if !self.check_args.is_empty() {
            self.check_args
        } else {
            default_check_args
        };
        let test_args = if !self.test_args.is_empty() {
            self.test_args
        } else {
            default_test_args
        };
        let fmt_args = if !self.fmt_args.is_empty() {
            self.fmt_args
        } else {
            default_fmt_args
        };
        let clippy_args = if !self.clippy_args.is_empty() {
            self.clippy_args
        } else {
            default_clippy_args
        };

        let commands = vec![
            ChecksCommand::check(&working_dir, check_args),
            ChecksCommand::test(&working_dir, test_args),
            ChecksCommand::fmt(&working_dir, fmt_args),
            ChecksCommand::clippy(&working_dir, clippy_args),
        ];

        let failed_commands = commands
            .into_iter()
            .filter_map(|x| x.execute(start).ok()?)
            .collect::<Vec<String>>();

        if !failed_commands.is_empty() {
            println!();
            println!("Fails ({}):", failed_commands.len());
            for command in failed_commands {
                println!("{}", command);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct ChecksCommand {
    kind: CheckKind,
    command: process::Command,
    command_string: String,
}

impl ChecksCommand {
    fn check(working_dir: &path::Path, args: Vec<String>) -> Self {
        let mut command_string = String::from("cargo check");

        let mut command = process::Command::new("cargo");
        command.current_dir(working_dir).arg("check");

        for arg in args {
            command_string.push(' ');
            command_string.push_str(&arg);

            command.arg(arg);
        }

        Self {
            kind: CheckKind::Check,
            command,
            command_string,
        }
    }

    fn test(working_dir: &path::Path, args: Vec<String>) -> Self {
        let mut command_string = String::from("cargo test");

        let mut command = process::Command::new("cargo");
        command.current_dir(working_dir).arg("test");

        for arg in args {
            command_string.push(' ');
            command_string.push_str(&arg);

            command.arg(arg);
        }

        Self {
            kind: CheckKind::Test,
            command,
            command_string,
        }
    }

    fn fmt(working_dir: &path::Path, args: Vec<String>) -> Self {
        let mut command_string = String::from("cargo fmt");

        let mut command = process::Command::new("cargo");
        command.current_dir(working_dir).arg("fmt");

        for arg in args {
            command_string.push(' ');
            command_string.push_str(&arg);

            command.arg(arg);
        }

        Self {
            kind: CheckKind::Fmt,
            command,
            command_string,
        }
    }

    fn clippy(working_dir: &path::Path, args: Vec<String>) -> Self {
        let mut command_string = String::from("cargo clippy");

        let mut command = process::Command::new("cargo");
        command.current_dir(working_dir).arg("clippy");

        for arg in args {
            command_string.push(' ');
            command_string.push_str(&arg);

            command.arg(arg);
        }

        Self {
            kind: CheckKind::Clippy,
            command,
            command_string,
        }
    }

    fn execute(mut self, start: time::Instant) -> Result<Option<String>> {
        let pb = create_pb();

        match &self.kind {
            CheckKind::Check => pb.set_message("Checking package..."),
            CheckKind::Test => pb.set_message("Testing..."),
            CheckKind::Fmt => pb.set_message("Checking formatting..."),
            CheckKind::Clippy => pb.set_message("Checking lints..."),
        }

        let res = self.command.output()?.status.success();

        pb.inc(1);

        pb.set_style(generate_style(res));
        pb.finish_with_message(self.kind.generate_msg(start));

        if res {
            Ok(None)
        } else {
            Ok(Some(self.command_string))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CheckKind {
    Check,
    Test,
    Fmt,
    Clippy,
}

impl CheckKind {
    fn generate_msg(self, start: time::Instant) -> String {
        let mut message = String::new();

        match self {
            CheckKind::Check => message.push_str("check  "),
            CheckKind::Test => message.push_str("test   "),
            CheckKind::Fmt => message.push_str("fmt    "),
            CheckKind::Clippy => message.push_str("clippy "),
        };

        message.push_str(format!("({}s)", start.elapsed().as_secs()).as_str());

        message
    }
}

fn create_pb() -> ProgressBar {
    let pb = ProgressBar::new(1);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:5.white} {msg}")
            .progress_chars("#>-"),
    );

    pb
}

fn generate_style(is_success: bool) -> ProgressStyle {
    if is_success {
        ProgressStyle::default_bar()
            .template("{bar:5.green} {msg}")
            .progress_chars("#>-")
    } else {
        ProgressStyle::default_bar()
            .template("{bar:5.red} {msg}")
            .progress_chars("#>-")
    }
}
