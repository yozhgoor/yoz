use anyhow::{bail, Result};
use std::{env, path, process};

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
        .filter(Some("yoz"), log::LevelFilter::Debug)
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
        Opt::Dotfiles(args) => args.run(
            config.default_repository_path,
            config.config_files_dir,
            config.config_repository_url,
            config.temporary_project_path,
            config.default_editor,
            config.default_terminal,
            config.default_bg_position,
            config.default_bg_file_path,
            config.default_fonts,
            config.default_fonts_size,
            config.default_browser,
            config.default_net_device,
            config.default_home_symbol,
        ),
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

fn value_or_default<T>(
    value: Option<T>,
    default: Option<T>,
    config_value: impl AsRef<str>,
) -> Result<T> {
    if let Some(value) = value {
        Ok(value)
    } else if let Some(default) = default {
        Ok(default)
    } else {
        bail!(
            "Please configure `{}` in your config file",
            config_value.as_ref()
        )
    }
}

fn values_or_default<T>(
    value: Vec<T>,
    default: Vec<T>,
    config_value: impl AsRef<str>,
) -> Result<Vec<T>> {
    if !value.is_empty() {
        Ok(value)
    } else if !default.is_empty() {
        Ok(default)
    } else {
        bail!(
            "Please configure `{}` in your config file",
            config_value.as_ref()
        )
    }
}

fn program_or_default(
    program: Option<String>,
    default: Option<String>,
    config_value: impl AsRef<str>,
) -> Result<process::Command> {
    let program = if let Some(program) = program {
        program
    } else if let Some(program) = default {
        program
    } else {
        bail!(
            "Please configure `{}` in your config file",
            config_value.as_ref()
        );
    };
    Ok(process::Command::new(program))
}
