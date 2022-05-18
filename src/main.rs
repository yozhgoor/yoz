use anyhow::{bail, Result};
use std::{env, path};

mod add;
mod background;
mod checks;
mod config;
mod dotfiles;
mod install;
mod launch;
mod license;
mod new;
mod screen;
mod shot;
mod update;
mod workflow;

use crate::config::Config;

#[derive(Debug, clap::Parser)]
#[clap(
    about = "This project aims to help my workflow.\n\nDon't expect any kind of stability there."
)]
enum Opt {
    Add(add::Add),
    Background(background::Background),
    Checks(checks::Checks),
    Dotfiles(dotfiles::Dotfiles),
    Install(install::Install),
    Launch(launch::Launch),
    New(new::New),
    Screen(screen::Screen),
    Shot(shot::Shot),
    Update(update::Update),
}

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(Some("yoz"), log::LevelFilter::Info)
        .init();

    let opt: Opt = clap::Parser::parse();

    let config = match Config::get_or_create() {
        Ok(config) => config,
        Err(err) => {
            bail!("an error occurred with the config file: {}", err);
        }
    };

    match opt {
        Opt::Add(args) => args.run(config.default_full_name),
        Opt::Background(args) => args.run(config.default_bg_file_path, config.default_bg_position),
        Opt::Checks(args) => args.run(
            config.default_check_args,
            config.default_test_args,
            config.default_fmt_args,
            config.default_clippy_args,
        ),
        Opt::Dotfiles(args) => args.run(),
        Opt::Install(args) => args.run(config.aur_dir),
        Opt::Launch(args) => args.run(config.default_editor, config.default_terminal),
        Opt::New(args) => args.run(config.default_full_name),
        Opt::Screen(args) => args.run(config.main_monitor, config.external_monitor),
        Opt::Shot(args) => args.run(),
        Opt::Update(args) => args.run(config.aur_dir),
    }
}

fn set_working_dir(path: Option<path::PathBuf>) -> Result<path::PathBuf> {
    let working_dir = match path {
        Some(path) if path.exists() => path,
        _ => env::current_dir().expect("cannot get current directory"),
    };

    Ok(working_dir)
}
