use crate::set_working_dir;
use anyhow::{bail, ensure, Result};
use std::{path, process};

/// Launch a program and a terminal at the same working directory.
#[derive(Debug, clap::Parser)]
pub struct Launch {
    /// Working directory of the processes.
    path: Option<path::PathBuf>,
    /// Launch the given command.
    ///
    /// If nothing is provided, `launch_command` from your config file
    /// would be used.
    #[clap(short = 'x', long)]
    command: Vec<String>,
    /// .
    #[clap(short = 't', long)]
    terminal: Vec<String>,
    /// Start only a terminal at the given working directory.
    #[clap(short = 'c', long)]
    no_command: bool,
}

impl Launch {
    pub fn run(
        self,
        default_launch_command: Vec<String>,
        default_terminal_command: Vec<String>,
    ) -> Result<()> {
        let working_dir = set_working_dir(self.path)?;

        let main_process = if self.no_command {
            None
        } else if self.command.is_empty() {
            if !default_launch_command.is_empty() {
                let mut it = default_launch_command.iter();
                let mut main_process = process::Command::new(it.next().expect("it is not empty"));
                main_process.current_dir(&working_dir);
                main_process.args(it);

                log::info!("Launching the default command");
                Some(main_process)
            } else {
                bail!("Please configure `launch_command` in your config file");
            }
        } else {
            let mut it = self.command.iter();
            let mut main_process = process::Command::new(it.next().expect("it is not empty"));
            main_process.current_dir(&working_dir);
            main_process.args(it);

            log::info!("Launching given command");
            Some(main_process)
        };

        let terminal_process = if !self.terminal.is_empty() {
            let mut it = self.terminal.iter();
            let mut terminal_process =
                process::Command::new(it.next().expect("self.terminal cannot be empty"));
            terminal_process.args(it);

            log::info!("Launching given terminal command");
            Some(terminal_process)
        } else if !default_terminal_command.is_empty() {
            let mut it = default_terminal_command.iter();
            let mut terminal_process =
                process::Command::new(it.next().expect("default_terminal_command cannot be empty"));
            terminal_process.current_dir(&working_dir);
            terminal_process.args(it);

            log::info!("Launching default terminal command");
            Some(terminal_process)
        } else {
            bail!("Please configure `terminal_command` in your config file");
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
