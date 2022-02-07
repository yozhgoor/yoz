use std::{env, process};

#[derive(clap::Parser)]
struct Opt {
    /// Path used by subcommands
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(clap::Parser)]
enum SubCommand {
    /// Launch a given program and open a new terminal at the same current
    /// directory.
    Launch {
        /// Launch the given command.
        ///
        /// If nothing is
        #[clap(short = 'a', long = "args")]
        /// The arguments given to the launched program.
        command: Vec<String>,
        /// Do not launch terminal along the launched program.
        #[clap(long)]
        no_terminal: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt.cmd {
        SubCommand::Launch {
            command,
            no_terminal,
        } => {
            let working_dir = if let Some(current_dir) = opt.path {
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
