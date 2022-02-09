#[derive(clap::Parser)]
enum Opt {
    /// Create a new Rust project with some defaults.
    New {
        /// Path where the project will be created (the name will be appended).
        path: Option<std::path::PathBuf>,
        /// Name of the Rust project.
        name: String,
        #[clap(long)]
        /// Create a new Rust library.
        lib: bool,
        /// Create a new Rust xtask project.
        #[clap(long)]
        xtask: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::New { path, name, lib, xtask } => {
            run_new(path, name, lib, xtask)
        }
    }
}

fn run_new(path: Option<std::path::PathBuf>, name: String, lib: bool, xtask: bool) -> anyhow::Result<()> {
        if working_dir.join(&name).exists() {
                panic!("destination already exists");
            }

            if !lib && !xtask {
                anyhow::ensure!(
                    std::process::Command::new("cargo")
                        .current_dir(&working_dir)
                        .args(["new", &name])
                        .status()
                        .expect("`cargo new` command failed")
                        .success(),
                    "cannot create new project"
                );
            } else if lib && !xtask {
                anyhow::ensure!(
                    std::process::Command::new("cargo")
                        .current_dir(&working_dir)
                        .args(["new", "--lib", &name])
                        .status()
                        .expect("`cargo new` command failed")
                        .success(),
                    "cannot create new project"
                );
            } else if !lib && xtask {
                todo!("a new binary project using xtask");
            } else if xtask && lib {
                todo!("a new library project using xtask");
            }

            Ok(())
}
