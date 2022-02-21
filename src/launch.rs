use crate::set_working_dir;
use anyhow::{ensure, Result};
use std::{path, process};

/// Launch a given program and open a new terminal at the same current
/// directory.
#[derive(clap::Parser)]
pub struct Launch {
    /// Working directory of the processes.
    path: Option<path::PathBuf>,
    /// Launch the given command.
    ///
    /// If nothing is provided, `nvim .` will be used.
    #[clap(short = 'x', long = "command")]
    command: Vec<String>,
    /// Do not launch terminal along the launched program.
    #[clap(long)]
    no_terminal: bool,
}

impl Launch {
    pub fn run(self) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let mut main_process = if self.command.is_empty() {
            let mut main_process = process::Command::new("nvim");
            main_process.current_dir(&working_dir);
            main_process.arg(".");

            main_process
        } else {
            let mut it = self.command.iter();
            let mut main_process = process::Command::new(it.next().unwrap());
            main_process.current_dir(&working_dir);
            main_process.args(it);

            main_process
        };

        let terminal_process = if !self.no_terminal {
            match process::Command::new("alacritty")
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

        ensure!(
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
}
