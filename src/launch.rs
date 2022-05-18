use crate::set_working_dir;
use anyhow::{bail, ensure, Result};
use std::{path::PathBuf, process};

/// Launch the editor and a terminal at the same working directory.
#[derive(Debug, clap::Parser)]
pub struct Launch {
    /// Working directory of the processes.
    path: Option<PathBuf>,
    /// Program to use as the editor.
    #[clap(long)]
    editor: Option<String>,
    #[clap(long)]
    terminal: Option<String>,
    /// Start only a terminal at the given working directory.
    #[clap(short = 't', long)]
    terminal_only: bool,
}

impl Launch {
    pub fn run(
        self,
        default_editor: Option<PathBuf>,
        default_terminal: Option<PathBuf>,
    ) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let editor_process = if !self.terminal_only {
            if let Some(program) = self.editor {
                let mut process = process::Command::new(program);
                process.current_dir(&working_dir);
                process.arg(".");

                log::info!("Launching the given editor");
                Some(process)
            } else if let Some(program) = default_editor {
                let mut process = process::Command::new(program);
                process.current_dir(&working_dir);
                process.arg(".");

                log::info!("Launching the default editor");
                Some(process)
            } else {
                bail!("Please configure `editor` in your config file");
            }
        } else {
            None
        };

        let mut terminal_process = if let Some(program) = self.terminal {
            let mut process = process::Command::new(program);
            process.current_dir(&working_dir);
            process.arg(".");

            log::info!("Launching the given terminal");
            process
        } else if let Some(program) = default_terminal {
            let mut process = process::Command::new(program);
            process.current_dir(&working_dir);
            process.args(["--working-directory", "."]);

            log::info!("Launching the default terminal");
            process
        } else {
            bail!("Please configure `terminal` in your config file");
        };

        if let Some(mut editor) = editor_process {
            let terminal = match terminal_process.spawn() {
                Ok(child) => Some(child),
                Err(err) => {
                    log::error!("an error occurred when launching the terminal: {}", err);
                    None
                }
            };

            ensure!(
                editor.status().expect("cannot launch editor").success(),
                "launch command failed"
            );

            if let Some(mut child) = terminal {
                child.kill()?;
                child.wait()?;
            }
        } else {
            ensure!(terminal_process.spawn().is_ok(), "cannot launch terminal");
        }

        Ok(())
    }
}
