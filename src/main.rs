#[derive(clap::Parser)]
enum Opt {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// Working directory of the processes.
        path: Option<std::path::PathBuf>,
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
    Checks {
        /// Path of the project that will be checked.
        ///
        /// This path must point to a Rust project.
        path: Option<std::path::PathBuf>,
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
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(Some("yoz"), log::LevelFilter::Info)
        .init();

    match opt {
        Opt::Launch {
            path,
            command,
            no_terminal,
        } => {
            let working_dir = set_working_dir(path)?;

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
                log::info!("Use the command directly instead");
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

            Ok(())
        }
        Opt::Checks {
            path,
            check,
            fmt,
            test,
            clippy,
            clean,
        } => {
            let working_dir = set_working_dir(path)?;

            if clean {
                if std::process::Command::new("cargo")
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

            let mut check = if !check.is_empty() {
                let mut command = std::process::Command::new("cargo");
                command.current_dir(&working_dir).args(check);

                command
            } else {
                let mut command = std::process::Command::new("cargo");
                command
                    .current_dir(&working_dir)
                    .args(["check", "--workspace", "--all-features"]);

                command
            };

            let mut test = if !test.is_empty() {
                let mut command = std::process::Command::new("cargo");
                command.current_dir(&working_dir).args(test);

                command
            } else {
                let mut command = std::process::Command::new("cargo");
                command
                    .current_dir(&working_dir)
                    .args(["test", "--workspace", "--all-features"]);

                command
            };

            let mut fmt = if !fmt.is_empty() {
                let mut command = std::process::Command::new("cargo");
                command.current_dir(&working_dir).args(fmt);

                command
            } else {
                let mut command = std::process::Command::new("cargo");
                command
                    .current_dir(&working_dir)
                    .args(["fmt", "--all", "--check"]);

                command
            };

            let mut clippy = if !clippy.is_empty() {
                let mut command = std::process::Command::new("cargo");
                command.current_dir(&working_dir).args(clippy);

                command
            } else {
                let mut command = std::process::Command::new("cargo");
                command
                    .current_dir(&working_dir)
                    .args(["clippy", "--all", "--tests", "--", "-D", "warnings"]);

                command
            };

            if check.output()?.status.success() {
                log::info!("cargo check : Ok");
            } else {
                log::error!("cargo check : Nope");
            };

            if test.output()?.status.success() {
                log::info!("cargo test  : Ok");
            } else {
                log::error!("cargo test  : Nope");
            };

            if fmt.output()?.status.success() {
                log::info!("cargo fmt   : Ok");
            } else {
                log::error!("cargo fmt   : Nope");
            };

            if clippy.output()?.status.success() {
                log::info!("cargo clippy: Ok");
            } else {
                log::error!("cargo clippy: Nope");
            };

            Ok(())
        }
    }
}

fn set_working_dir(path: Option<std::path::PathBuf>) -> anyhow::Result<std::path::PathBuf> {
    let working_dir = match path {
        Some(path) if path.exists() => path,
        _ => std::env::current_dir().expect("cannot get current directory"),
    };

    Ok(working_dir)
}
