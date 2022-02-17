/// Create a new Rust project with some defaults.
#[derive(clap::Parser)]
pub struct New {
    /// Name of the Rust project.
    name: String,
    #[clap(short = 'p', long)]
    /// Path where the project will be created (the name will be appended).
    path: Option<std::path::PathBuf>,
    /// Full name used in the licenses.
    #[clap(long)]
    full_name: Option<String>,
    /// Create a new Rust library.
    #[clap(short = 'l', long)]
    lib: bool,
    /// Create a new Rust xtask project.
    #[clap(short = 'x', long)]
    xtask: bool,
    /// Do not add licenses.
    #[clap(long)]
    no_license: bool,
    /// Do not add CI.
    #[clap(long)]
    no_ci: bool,
    /// Do not add CI tests for Windows.
    #[clap(long)]
    no_windows: bool,
    /// Do not add CI tests for OSX.
    #[clap(long)]
    no_osx: bool,
}

impl New {
    pub fn run(self) -> anyhow::Result<()> {
        let working_dir = crate::set_working_dir(self.path)?;

        let project_dir_path = working_dir.join(&self.name);
        std::fs::create_dir(&project_dir_path)?;

        if !self.no_license {
            add_licenses(&project_dir_path, self.full_name)?;
        }

        if !self.no_ci {
            let workflows_dir = project_dir_path.join(".github").join("workflows");
            std::fs::create_dir_all(&workflows_dir)?;

            if self.lib {
                add_lib_ci(&workflows_dir, self.no_windows, self.no_osx)?;
            } else {
                add_bin_ci(&workflows_dir, &self.name, self.no_windows, self.no_osx)?;
            }
        }

        Ok(())
    }
}

use chrono::Datelike;

fn add_licenses(
    project_dir_path: &std::path::Path,
    full_name: Option<String>,
) -> anyhow::Result<()> {
    let year = chrono::Local::now().date().year();
    let full_name = full_name.unwrap_or_else(|| "Yohan Boogaert".to_string());

    std::fs::write(
        project_dir_path.join("LICENSE.MIT"),
        format!(
            include_str!("../templates/mit"),
            year = year,
            full_name = full_name
        ),
    )?;
    std::fs::write(
        project_dir_path.join("LICENSE.Apache-2.0"),
        format!(
            include_str!("../templates/apache"),
            year = year,
            full_name = full_name
        ),
    )?;

    Ok(())
}

fn add_lib_ci(
    workflows_dir: &std::path::Path,
    no_windows: bool,
    no_osx: bool,
) -> anyhow::Result<()> {
    let main_workflow = generate_main_workflow(no_windows, no_osx);
    let pr_workflow = generate_pr_workflow(no_windows, no_osx);

    std::fs::write(workflows_dir.join("main.yml"), main_workflow)?;
    std::fs::write(workflows_dir.join("pr.yml"), pr_workflow)?;

    Ok(())
}

fn add_bin_ci(
    workflows_dir: &std::path::Path,
    project_name: &str,
    no_windows: bool,
    no_osx: bool,
) -> anyhow::Result<()> {
    let main_workflow = generate_main_workflow(no_windows, no_osx);
    let pr_workflow = generate_pr_workflow(no_windows, no_osx);
    let release_workflow = generate_release_workflow(project_name, no_windows, no_osx);

    std::fs::write(workflows_dir.join("main.yml"), main_workflow)?;
    std::fs::write(workflows_dir.join("pr.yml"), pr_workflow)?;
    std::fs::write(workflows_dir.join("release.yml"), release_workflow)?;

    Ok(())
}

fn generate_main_workflow(_no_windows: bool, _no_osx: bool) -> String {
    todo!("generate main workflow");
}

fn generate_pr_workflow(_no_windows: bool, _no_osx: bool) -> String {
    todo!("generate pr workflow");
}

fn generate_release_workflow(_project_name: &str, _no_windows: bool, _no_osx: bool) -> String {
    todo!("generate release workflow");
}
