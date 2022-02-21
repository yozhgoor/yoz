use crate::{license, set_working_dir, workflow};
use anyhow::Result;
use std::path;

/// Add useful content to your Rust project.
#[derive(clap::Parser)]
pub struct Add {
    /// Path where you want to add the content.
    #[clap(short = 'p', long)]
    path: Option<path::PathBuf>,
    /// Add licenses to the project.
    #[clap(long)]
    licenses: bool,
    /// Full name used in the licenses.
    #[clap(long)]
    full_name: Option<String>,
    /// Add CI to the project.
    #[clap(long)]
    ci: bool,
    /// Add CI for a library instead of a binary.
    #[clap(long)]
    lib: bool,
    /// Do not add CI tests for Windows.
    #[clap(long)]
    no_windows: bool,
    /// Do not add CI tests for OSX.
    #[clap(long)]
    no_osx: bool,
}

impl Add {
    pub fn run(self) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        if self.licenses {
            log::info!("Generating licenses");
            license::add_licenses(&working_dir, self.full_name)?;
        } else if self.ci {
            log::info!("Generating CI files");
            if self.lib {
                workflow::add_lib_ci(&working_dir, self.no_windows, self.no_osx)?;
            } else {
                workflow::add_bin_ci(
                    &working_dir,
                    &cargo_metadata::MetadataCommand::new()
                        .exec()?
                        .root_package()
                        .expect("cannot resolve root package")
                        .name,
                    self.no_windows,
                    self.no_osx,
                )?;
            }
        } else {
            log::error!("Please select something to add");
        }

        Ok(())
    }
}
