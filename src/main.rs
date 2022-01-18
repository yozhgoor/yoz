use anyhow::{ensure, Result};
use std::{env, process};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Launch {
        program: String,
        #[structopt(long)]
        args: Vec<String>,
        #[structopt(long)]
        no_terminal: bool,
    },
    Update {
        target: String,
    },
    Install,
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
            Command::Update { target } => match target.as_str() {
                "linux" => {
                    ensure!(
                        process::Command::new("pacman")
                            .args(["-Syu"])
                            .status()
                            .expect("cannot launch pacman")
                            .success(),
                        "cannot update linux"
                    );
                }
                "cargo-temp" => {
                    todo!();
                }
                "neovim" => {
                    todo!();
                }
                "vscodium" => {
                    todo!();
                }
                _ => println!("not implemented"),
            },
            Command::Install => {
                todo!();
            }
        }
    }

    Ok(())
}
