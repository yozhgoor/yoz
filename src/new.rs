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
            log::info!("Generating licenses");
            add_licenses(&project_dir_path, self.full_name)?;
        }

        if !self.no_ci {
            let workflows_dir = project_dir_path.join(".github").join("workflows");
            std::fs::create_dir_all(&workflows_dir)?;

            if self.lib {
                log::info!("Generating lib's CI");
                add_lib_ci(&workflows_dir, self.no_windows, self.no_osx)?;
            } else {
                log::info!("Generating bin's CI");
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
            include_str!("../templates/licenses/mit"),
            year = year,
            full_name = full_name
        ),
    )?;
    std::fs::write(
        project_dir_path.join("LICENSE.Apache-2.0"),
        format!(
            include_str!("../templates/licenses/apache"),
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
    let main_workflow = generate_main_workflow(no_windows, no_osx)?;
    let pr_workflow = generate_pr_workflow(no_windows, no_osx)?;

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
    let main_workflow = generate_main_workflow(no_windows, no_osx)?;
    let pr_workflow = generate_pr_workflow(no_windows, no_osx)?;
    let release_workflow = generate_release_workflow(project_name, no_windows, no_osx)?;

    std::fs::write(workflows_dir.join("main.yml"), main_workflow)?;
    std::fs::write(workflows_dir.join("pr.yml"), pr_workflow)?;
    std::fs::write(workflows_dir.join("release.yml"), release_workflow)?;

    Ok(())
}

use std::fmt::Write;

fn generate_main_workflow(no_windows: bool, no_osx: bool) -> anyhow::Result<String> {
    let header = "name: main

on:
  push:
    branches: [ main ]
  schedule:
    - cron: 0 0 1 * *

env:
  CARGO_TERM_COLOR: always

jobs:
";

    let mut workflow = String::new();

    write!(workflow, "{}", header,)?;

    write!(
        workflow,
        include_str!("../templates/ci/test"),
        job_name = "test",
        platform = "ubuntu-latest"
    )?;

    if !no_windows {
        write!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-windows",
            platform = "windows-latest"
        )?;
    }

    if !no_osx {
        write!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-osx",
            platform = "windows-latest"
        )?;
    }

    Ok(workflow)
}

fn generate_pr_workflow(no_windows: bool, no_osx: bool) -> anyhow::Result<String> {
    let header = "name: PR

on:
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
";

    let mut workflow = String::new();

    write!(workflow, "{}", header,)?;
    write!(
        workflow,
        include_str!("../templates/ci/test_and_lint"),
        job_name = "test-and-lint",
        platform = "ubuntu-latest"
    )?;

    if !no_windows {
        write!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-windows",
            platform = "windows-latest"
        )?;
    }

    if !no_osx {
        write!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-osx",
            platform = "windows-latest"
        )?;
    }

    Ok(workflow)
}

fn generate_release_workflow(
    project_name: &str,
    no_windows: bool,
    no_osx: bool,
) -> anyhow::Result<String> {
    let build_linux_name = "build-linux";
    let build_windows_name = "build-windows";
    let build_osx_name = "build-osx";

    let header = "name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
";

    let mut workflow = String::new();

    write!(workflow, "{}", header,)?;
    write!(
        workflow,
        include_str!("../templates/ci/release_build_with_strip"),
        job_name = build_linux_name,
        platform = "ubuntu-latest",
        target = "x86_64-unknown-linux-gnu",
        build_title = "Build release (Windows)",
        build_name = build_linux_name,
        project_name = project_name,
    )?;

    if !no_windows {
        write!(
            workflow,
            include_str!("../templates/ci/release_build"),
            job_name = build_windows_name,
            platform = "windows-latest",
            target = "x86_64-windows-msvc",
            build_title = "Build release (Windows)",
            build_name = build_windows_name,
            project_name = project_name,
        )?;
    }

    if !no_osx {
        write!(
            workflow,
            include_str!("../templates/ci/release_build"),
            job_name = build_osx_name,
            platform = "macos-latest",
            target = "x86_64-apple-darwin",
            build_title = "Build release (OSX)",
            build_name = build_osx_name,
            project_name = project_name,
        )?;
    }

    let needs = if no_windows && no_osx {
        format!("[{}]", build_linux_name)
    } else if !no_windows && no_osx {
        format!("[{}, {}]", build_linux_name, build_windows_name)
    } else if no_windows && !no_osx {
        format!("[{}, {}]", build_linux_name, build_osx_name)
    } else {
        format!(
            "[{}, {}, {}]",
            build_linux_name, build_windows_name, build_osx_name
        )
    };

    write!(
        workflow,
        include_str!("../templates/ci/release_start"),
        needs = needs,
    )?;

    write!(
        workflow,
        include_str!("../templates/ci/release_step"),
        build_name = build_linux_name,
        build_path = build_linux_name,
        project_name = project_name,
        platform = "linux-x86_64"
    )?;

    if !no_windows {
        write!(
            workflow,
            include_str!("../templates/ci/release_step"),
            build_name = build_windows_name,
            build_path = build_windows_name,
            project_name = project_name,
            platform = "windows-x86_64"
        )?;
    }

    if !no_osx {
        write!(
            workflow,
            include_str!("../templates/ci/release_step"),
            build_name = build_osx_name,
            build_path = build_osx_name,
            project_name = project_name,
            platform = "osx-x86_64"
        )?;
    }

    let files = if no_windows && no_osx {
        format!(
            "|
            {}/*
",
            build_linux_name
        )
    } else if !no_windows && no_osx {
        format!(
            "|
            {}/*
            {}/*
",
            build_linux_name, build_windows_name
        )
    } else if no_windows && !no_osx {
        format!(
            "|
            {}/*
            {}/*
",
            build_linux_name, build_osx_name
        )
    } else {
        format!(
            "|
            {}/*
            {}/*
            {}/*
",
            build_linux_name, build_windows_name, build_osx_name
        )
    };

    write!(
        workflow,
        include_str!("../templates/ci/release_end"),
        files = files
    )?;

    Ok(workflow)
}
