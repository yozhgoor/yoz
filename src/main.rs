mod checks;
mod launch;
mod new;

#[derive(clap::Parser)]
enum Opt {
    New(new::New),
    Launch(launch::Launch),
    Checks(checks::Checks),
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(Some("yoz"), log::LevelFilter::Info)
        .init();

    match opt {
        Opt::Launch(args) => args.run(),
        Opt::Checks(args) => args.run(),
        Opt::New(args) => args.run(),
    }
}

fn set_working_dir(path: Option<std::path::PathBuf>) -> anyhow::Result<std::path::PathBuf> {
    let working_dir = match path {
        Some(path) if path.exists() => path,
        _ => std::env::current_dir().expect("cannot get current directory"),
    };

    Ok(working_dir)
}
