use anyhow::{bail, Result};
use std::{env, path};

mod add;
mod checks;
mod config;
mod launch;
mod license;
mod new;
mod screen;
mod workflow;

use crate::config::Config;

#[derive(clap::Parser)]
enum Opt {
    Add(add::Add),
    Checks(checks::Checks),
    Launch(launch::Launch),
    New(new::New),
    Screen(screen::Screen),
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(Some("yoz"), log::LevelFilter::Info)
        .init();

    let config = match Config::get_or_create() {
        Ok(config) => config,
        Err(err) => {
            bail!("an error occurred with the config file: {}", err);
        }
    };

    match opt {
        Opt::Add(args) => args.run(config.default_full_name),
        Opt::Checks(args) => args.run(),
        Opt::Launch(args) => args.run(),
        Opt::New(args) => args.run(config.default_full_name),
        Opt::Screen(args) => args.run(
            config.main_monitor,
            config.external_monitor,
            config.bg_file_path,
            config.bg_position,
        ),
    }
}

fn set_working_dir(path: Option<path::PathBuf>) -> Result<path::PathBuf> {
    let working_dir = match path {
        Some(path) if path.exists() => path,
        _ => env::current_dir().expect("cannot get current directory"),
    };

    Ok(working_dir)
}
