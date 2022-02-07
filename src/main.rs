#[derive(clap::Parser)]
struct Opt {
    /// Path used by subcommands
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
    #[clap(short = 'l', long = "log")]
    log: bool,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(clap::Parser)]
enum SubCommand {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// Launch the given command.
        ///
        /// If nothing is
        #[clap(short = 'a', long = "args")]
        /// The arguments given to the launched program.
        command: Vec<String>,
        /// Do not launch terminal along the launched program.
        #[clap(long)]
        no_terminal: bool,
    },
    /// Launch the checks command needed for a Rust project.
    Check,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    if opt.log {
        env_logger::builder()
        .filter(Some("yoz"), log::LevelFilter::Error)
        .init();
    }

    match opt.cmd {
        SubCommand::Launch {
            command,
            no_terminal,
        } => {
            let working_dir = if let Some(current_dir) = opt.path {
                current_dir
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
        SubCommand::Check => {
            log::info!("Launching `cargo check`");
            match std::process::Command::new("cargo")
                    .arg("check")
                    .status() {
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
                    .args(["test", "--workspace"])
                    .status() {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo check: Ok.")
                    } else {
                        log::error!("`cargo test --workspace` wasn't successful: {}", exit_status)
                    }
                }
                Err(err) => {
                    log::error!("cannot launch `cargo test --workspace`: {}", err)
                }
            }

            log::info!("Launching `cargo fmt --all -- --check`");
            match std::process::Command::new("cargo")
                    .args(["fmt", "--all", "--", "--check"])
                    .status() {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo fmt --all -- --check`: Ok.")
                    } else {
                        log::error!("`cargo fmt --all -- --check` wasn't successful: {}", exit_status)
                    }
                }
                Err(err) => {
                    log::error!("cannot launch `cargo fmt --all -- --check`: {}", err)
                }
            }

            log::info!("Launching `cargo clippy -- -D warnings");
            match std::process::Command::new("cargo")
                    .args(["clippy", "--", "-D", "warnings"])
                    .status() {
                Ok(exit_status) => {
                    if exit_status.success() {
                        log::info!("`cargo clippy -- -D warnings`: Ok.")
                    } else {
                        log::error!("`cargo clippy -- -D warnings` wasn't successful: {}", exit_status)
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
