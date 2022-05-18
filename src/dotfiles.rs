use anyhow::{bail, ensure, Result};
use std::{path::PathBuf, process};

/// Generate config files from dotfiles
#[derive(Debug, clap::Parser)]
pub struct Dotfiles {
    repository_path: Option<PathBuf>,
    config_files_dir: Option<PathBuf>,
    config_repository_url: Option<String>,
}

impl Dotfiles {
    pub fn run(
        self,
        default_repository_path: Option<PathBuf>,
        default_config_files_dir: Option<PathBuf>,
        default_config_repository_url: Option<String>,
    ) -> Result<()> {
        let repository_path = if let Some(path) = self.repository_path {
            path
        } else if let Some(path) = default_repository_path {
            path
        } else {
            bail!("Please configure `repository_path` in your config file");
        };

        let config_files_path = if let Some(dir) = self.config_files_dir {
            repository_path.join(dir)
        } else if let Some(dir) = default_config_files_dir {
            repository_path.join(dir)
        } else {
            bail!("Please configure `config_files_path` in your config file");
        };

        let config_repository_url = if let Some(url) = self.config_repository_url {
            url
        } else if let Some(url) = default_config_repository_url {
            url
        } else {
            bail!("Please configure `config_repository_url` in your config file");
        };

        let config_files = ConfigFiles::get_or_download(
            config_files_path,
            config_repository_url,
            repository_path,
        )?;

        config_files.generate_cargo_temp()?;
        config_files.generate_i3()?;
        config_files.generate_i3status()?;
        config_files.generate_nvim()?;
        config_files.generate_starship()?;

        Ok(())
    }
}

struct ConfigFiles {
    cargo_temp: PathBuf,
    i3: PathBuf,
    i3status: PathBuf,
    nvim: PathBuf,
    starship: PathBuf,
}

impl ConfigFiles {
    fn get_or_download(
        config_files_path: PathBuf,
        config_repository_url: String,
        repository_path: PathBuf,
    ) -> Result<Self> {
        let config_files_path = if config_files_path.exists() {
            config_files_path
        } else {
            log::info!("Downloading config files");
            ensure!(
                process::Command::new("git")
                    .arg("clone")
                    .arg(config_repository_url)
                    .current_dir(repository_path)
                    .status()?
                    .success(),
                "cannot download config files"
            );

            config_files_path
        };

        Ok(Self {
            cargo_temp: config_files_path.join("cargo-temp"),
            i3: config_files_path.join("i3"),
            i3status: config_files_path.join("i3status"),
            nvim: config_files_path.join("nvim"),
            starship: config_files_path.join("starship"),
        })
    }

    fn generate_cargo_temp(&self) -> Result<()> {
        Ok(())
    }

    fn generate_i3(&self) -> Result<()> {
        Ok(())
    }

    fn generate_i3status(&self) -> Result<()> {
        Ok(())
    }

    fn generate_nvim(&self) -> Result<()> {
        Ok(())
    }

    fn generate_starship(&self) -> Result<()> {
        Ok(())
    }
}
