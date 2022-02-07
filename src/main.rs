use anyhow::{ensure, Result};
use std::{env, process};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// The program that will be launched.
        program: String,
        #[structopt(short = "a", long = "args")]
        /// The arguments given to the launched program.
        args: Vec<String>,
        /// Do not launch terminal along the launched program.
        #[structopt(long)]
        no_terminal: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    let working_dir = cli
        .path
        .unwrap_or_else(|| env::current_dir().expect("cannot get current directory"));

    if let Some(command) = cli.cmd {
        match command {
            Command::Launch {
                program,
                args,
                no_terminal,
            } => {
                let terminal_process = if !no_terminal {
                    match process::Command::new("alacritty")
                        .arg("--working-directory")
                        .arg(working_dir.as_os_str())
                        .spawn()
                    {
                        Ok(child) => Some(child),
                        Err(err) => {
                            println!("an error occurred when launching alacritty: {err}");
                            None
                        }
                    }
                } else {
                    println!("use directly {} instead", &program);
                    None
                };

                ensure!(
                    process::Command::new(&program)
                        .current_dir(&working_dir)
                        .args(args)
                        .status()
                        .expect("cannot launch {&program}")
                        .success(),
                    "launch command failed"
                );

                if let Some(mut child) = terminal_process {
                    child.kill()?;
                    child.wait()?;
                }
            }
        }
    }

    Ok(())
}
