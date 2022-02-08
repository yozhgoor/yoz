use std::{env, process};

#[derive(clap::Parser)]
enum Opt {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// path used to set the current directory of the processes.
        #[clap(parse(from_os_str))]
        path: Option<std::path::PathBuf>,
        /// Launch the given command.
        ///
        /// If nothing is provided, `nvim .` will be used.
        #[clap(short = 'x', long = "command")]
        /// The arguments given to the launched program.
        command: Vec<String>,
        /// Do not launch terminal along the launched program.
        #[clap(long)]
        no_terminal: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::Launch {
            path,
            command,
            no_terminal,
        } => {
            let working_dir = if let Some(current_dir) = path {
                current_dir
            } else {
                env::current_dir().expect("cannot get current directory")
            };

            let mut main_process = if command.is_empty() {
                let mut main_process = process::Command::new("nvim");
                main_process.current_dir(&working_dir);
                main_process.arg(".");

                main_process
            } else {
                let mut it = command.iter();
                let mut main_process = process::Command::new(it.next().unwrap());
                main_process.current_dir(&working_dir);
                main_process.args(it);

                main_process
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
                println!("Use the command directly instead");
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
