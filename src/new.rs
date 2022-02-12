/// Create a new Rust project with some defaults.
#[derive(clap::Parser)]
pub struct New {
    /// Name of the Rust project.
    name: String,
    #[clap(short = 'p', long)]
    /// Path where the project will be created (the name will be appended).
    path: Option<std::path::PathBuf>,
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
}

impl New {
    pub fn run(self) -> anyhow::Result<()> {
        let working_dir = crate::set_working_dir(self.path)?;

        let project_dir_path = working_dir.join(self.name);
        std::fs::create_dir(&project_dir_path)?;

        if !self.no_license {
            let year = chrono::Utc::today();
            let fullname = self
                .full_name
                .unwrap_or_else(|| "Yohan Boogaert".to_string());

            std::fs::write(
                &project_dir_path.join("LICENSE.mit"),
                format!(
                    include_str!("../templates/mit"),
                    year = year,
                    fullname = fullname
                ),
            )?;
            std::fs::write(
                &project_dir_path.join("LICENSE.Apache-2.0"),
                format!(
                    include_str!("../templates/apache"),
                    year = year,
                    fullname = fullname
                ),
            )?;
        }

        if !self.no_ci {
            todo!("write CI file");
        }

        Ok(())
    }
}
