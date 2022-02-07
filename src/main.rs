use anyhow;
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

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Launch {
            program,
            args,
            no_terminal,
        } => {

            let mut main_process = process::Command::new(&program);
            main_process.args(args);

            let working_dir = if let Some(current_dir) = main_process.get_current_dir() {
                current_dir.to_path_buf()
            } else {
                env::current_dir().expect("cannot get current directory")
            };

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

            anyhow::ensure!(
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
        }
    }

    Ok(())
}
