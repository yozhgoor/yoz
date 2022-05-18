use crate::{program_or_default, set_working_dir};
use anyhow::{ensure, Result};
use std::path::PathBuf;

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
        default_editor: Option<String>,
        default_terminal: Option<String>,
    ) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let editor_process = if !self.terminal_only {
            let mut process = program_or_default(self.editor, default_editor, "editor")?;
            process.current_dir(&working_dir);
            process.arg(".");

            Some(process)
        } else {
            None
        };

        let mut terminal_process = program_or_default(self.terminal, default_terminal, "terminal")?;
        terminal_process.current_dir(&working_dir);
        terminal_process.args(["--working-directory", "."]);

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
