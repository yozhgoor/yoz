use anyhow::Result;
use std::{env, path};

mod add;
mod checks;
mod launch;
mod license;
mod new;
mod workflow;

#[derive(clap::Parser)]
enum Opt {
    Add(add::Add),
    Checks(checks::Checks),
    Launch(launch::Launch),
    New(new::New),
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(Some("yoz"), log::LevelFilter::Info)
        .init();

    match opt {
        Opt::Add(args) => args.run(),
        Opt::Checks(args) => args.run(),
        Opt::Launch(args) => args.run(),
        Opt::New(args) => args.run(),
    }
}

fn set_working_dir(path: Option<path::PathBuf>) -> Result<path::PathBuf> {
    let working_dir = match path {
        Some(path) if path.exists() => path,
        _ => env::current_dir().expect("cannot get current directory"),
    };

    Ok(working_dir)
}
