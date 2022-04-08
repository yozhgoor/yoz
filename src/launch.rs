use crate::set_working_dir;
use anyhow::{ensure, Result};
use std::{path, process};

/// Launch a given program and open a new terminal at the same current
/// directory.
#[derive(Debug, clap::Parser)]
pub struct Launch {
    /// Working directory of the processes.
    path: Option<path::PathBuf>,
    /// Launch the given command.
    ///
    /// If nothing is provided, `nvim .` will be used.
    #[clap(short = 'x', long)]
    command: Vec<String>,
    /// Do not launch terminal along the launched program.
    #[clap(short = 't', long)]
    no_terminal: bool,
    /// Start only a terminal at the given working directory.
    #[clap(short = 'c', long)]
    no_command: bool,
}

impl Launch {
    pub fn run(self) -> Result<()> {
        log::debug!("{:?}", self);

        let working_dir = set_working_dir(self.path)?;

        let main_process = if self.no_command {
            log::info!("No command provided");
            None
        } else if self.command.is_empty() {
            let mut main_process = process::Command::new("nvim");
            main_process.current_dir(&working_dir);
            main_process.arg(".");

            log::info!("Launching Neovim");
            Some(main_process)
        } else {
            let mut it = self.command.iter();
            let mut main_process = process::Command::new(it.next().unwrap());
            main_process.current_dir(&working_dir);
            main_process.args(it);

            log::info!("Launching given command");
            Some(main_process)
        };

        let terminal_process = if !self.no_terminal {
            let mut terminal_process = process::Command::new("alacritty");
            terminal_process.arg("--working-directory").arg(working_dir);

            Some(terminal_process)
        } else {
            log::info!("Use the command directly instead");
            None
        };

        match (main_process, terminal_process) {
            (Some(mut main), Some(mut term)) => {
                let term_child = match term.spawn() {
                    Ok(child) => Some(child),
                    Err(err) => {
                        log::error!("an error occurred when launching alacritty: {}", err);
                        None
                    }
                };

                ensure!(
                    main.status().expect("cannot launch main process").success(),
                    "launch command failed"
                );

                if let Some(mut child) = term_child {
                    child.kill()?;
                    child.wait()?;
                }
            }
            (Some(mut main), None) => {
                ensure!(
                    main.status().expect("cannot launch main process").success(),
                    "main process failed",
                );
            }
            (None, Some(mut term)) => {
                ensure!(term.spawn().is_ok(), "terminal process failed",);
            }
            (None, None) => unimplemented!(),
        }

        Ok(())
    }
}
