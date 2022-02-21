#[derive(clap::Parser)]
pub struct Checks {
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
}

impl Checks {
    pub fn run(self) -> anyhow::Result<()> {
        let working_dir = crate::set_working_dir(self.path)?;

        if self.clean {
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

        let mut check = if !self.check.is_empty() {
            let mut command = std::process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.check);

            command
        } else {
            let mut command = std::process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["check", "--workspace", "--all-features"]);

            command
        };

        let mut test = if !self.test.is_empty() {
            let mut command = std::process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.test);

            command
        } else {
            let mut command = std::process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["test", "--workspace", "--all-features"]);

            command
        };

        let mut fmt = if !self.fmt.is_empty() {
            let mut command = std::process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.fmt);

            command
        } else {
            let mut command = std::process::Command::new("cargo");
            command
                .current_dir(&working_dir)
                .args(["fmt", "--all", "--check"]);

            command
        };

        let mut clippy = if !self.clippy.is_empty() {
            let mut command = std::process::Command::new("cargo");
            command.current_dir(&working_dir).args(self.clippy);

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
