#[derive(clap::Parser)]
enum Opt {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// Working directory of the processes.
        working_dir: Option<std::path::PathBuf>,
        /// Launch the given command.
        ///
        /// If nothing is provided, `nvim .` will be used.
        #[clap(short = 'x', long = "command")]
        command: Vec<String>,
        /// Do not launch terminal along the launched program.
        #[clap(long)]
        no_terminal: bool,
    },
    /// Launch the checks command needed for a Rust project.
    Check {
        /// Path of the project that will be checked.
        ///
        /// This path must point to a Rust project.
        working_dir: Option<std::path::PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();
    match opt {
        Opt::Launch {
            working_dir,
            command,
            no_terminal,
        } => {
            let working_dir = if let Some(path) = working_dir {
                if path.exists() {
                    path
                } else {
                    anyhow::bail!("{} doesn't exist", path.display());
                }
            } else {
                std::env::current_dir().expect("cannot get current directory")
            };

            let mut main_process = if command.is_empty() {
                let mut main_process = std::process::Command::new("nvim");
                main_process.current_dir(&working_dir);
                main_process.arg(".");

                main_process
            } else {
                let mut it = command.iter();
                let mut main_process = std::process::Command::new(it.next().unwrap());
                main_process.current_dir(&working_dir);
                main_process.args(it);

                main_process
            };

            let terminal_process = if !no_terminal {
                match std::process::Command::new("alacritty")
                    .arg("--working-directory")
                    .arg(working_dir.as_os_str())
                    .spawn()
                {
                    Ok(child) => Some(child),
                    Err(err) => {
                        println!("an error occurred when launching alacritty: {}", err);
                        None
                    }
                }
            } else {
                println!("Use the command directly instead");
                None
            };

            anyhow::ensure!(
                main_process
                    .status()
                    .expect("cannot launch main process")
                    .success(),
                "launch command failed"
            );

            if let Some(mut child) = terminal_process {
                child.kill()?;
                child.wait()?;
            }
        }
        Opt::Check { working_dir } => {
            let working_dir = if let Some(path) = working_dir {
                if path.exists() {
                    path
                } else {
                    anyhow::bail!("{} doesn't exist", path.display());
                }
            } else {
                std::env::current_dir().expect("cannot get current directory")
            };

            log::info!("Launching `cargo check`");
            match std::process::Command::new("cargo")
                .current_dir(&working_dir)
                .arg("check")
                .status()
            {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo check`: Ok.")
                    } else {
                        log::error!("`cargo check` wasn't successful {}", exit_status)
                    }
                }
                Err(err) => {
                    log::error!("Cannot launch `cargo check`: {}", err)
                }
            }

            log::info!("Launching `cargo test --workspace`");
            match std::process::Command::new("cargo")
                .current_dir(&working_dir)
                .args(["test", "--workspace"])
                .status()
            {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo check: Ok.")
                    } else {
                        log::error!(
                            "`cargo test --workspace` wasn't successful: {}",
                            exit_status
                        )
                    }
                }
                Err(err) => {
                    log::error!("cannot launch `cargo test --workspace`: {}", err)
                }
            }

            log::info!("Launching `cargo fmt --all -- --check`");
            match std::process::Command::new("cargo")
                .current_dir(&working_dir)
                .args(["fmt", "--all", "--", "--check"])
                .status()
            {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo fmt --all -- --check`: Ok.")
                    } else {
                        log::error!(
                            "`cargo fmt --all -- --check` wasn't successful: {}",
                            exit_status
                        )
                    }
                }
                Err(err) => {
                    log::error!("cannot launch `cargo fmt --all -- --check`: {}", err)
                }
            }

            log::info!("Launching `cargo clippy -- -D warnings");
            match std::process::Command::new("cargo")
                .current_dir(&working_dir)
                .args(["clippy", "--", "-D", "warnings"])
                .status()
            {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo clippy -- -D warnings`: Ok.")
                    } else {
                        log::error!(
                            "`cargo clippy -- -D warnings` wasn't successful: {}",
                            exit_status
                        )
                    }
                }
                Err(err) => {
                    log::error!("cannot launch `cargo clippy -- -D warnings`: {}", err)
                }
            }
        }
    }

    Ok(())
}
