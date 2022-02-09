/// Create a new Rust project with some defaults.
#[derive(clap::Parser)]
pub struct New {
    /// Path where the project will be created (the name will be appended).
    path: Option<std::path::PathBuf>,
    /// Name of the Rust project.
    name: String,
    /// Create a new Rust library.
    #[clap(long)]
    lib: bool,
    /// Create a new Rust xtask project.
    #[clap(long)]
    xtask: bool,
}

impl New {
    pub fn run(self) -> anyhow::Result<()> {
        todo!("New: run");
    }
}
