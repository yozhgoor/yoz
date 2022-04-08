use crate::{license, set_working_dir, workflow};
use anyhow::{bail, Result};
use std::path;

/// Add useful content to your Rust project.
#[derive(Debug, clap::Parser)]
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
    ///
    /// Generate needed files for a binary.
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
    pub fn run(self, default_full_name: Option<String>) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let full_name = if let Some(full_name) = self.full_name {
            full_name
        } else if let Some(full_name) = default_full_name {
            full_name
        } else {
            bail!("Please configure `full_name` in your config file")
        };

        if self.licenses {
            log::info!("Generating licenses");
            license::add_licenses(&working_dir, full_name)?;
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
