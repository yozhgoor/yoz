use std::fmt::Write;

pub fn add_lib_ci(
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

pub fn add_bin_ci(
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

    writeln!(
        workflow,
        include_str!("../templates/ci/test"),
        job_name = "test",
        platform = "ubuntu-latest"
    )?;

    if !no_windows {
        writeln!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-windows",
            platform = "windows-latest"
        )?;
    }

    if !no_osx {
        writeln!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-osx",
            platform = "windows-latest"
        )?;
    }

    let mut workflow = workflow.trim_end().to_string();

    writeln!(workflow)?;

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
    writeln!(
        workflow,
        include_str!("../templates/ci/test_and_lint"),
        job_name = "test-and-lint",
        platform = "ubuntu-latest"
    )?;

    if !no_windows {
        writeln!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-windows",
            platform = "windows-latest"
        )?;
    }

    if !no_osx {
        writeln!(
            workflow,
            include_str!("../templates/ci/test"),
            job_name = "test-osx",
            platform = "windows-latest"
        )?;
    }

    let mut workflow = workflow.trim_end().to_string();

    writeln!(workflow)?;

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
    writeln!(
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
        writeln!(
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
        writeln!(
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

    writeln!(
        workflow,
        include_str!("../templates/ci/release_step"),
        build_name = build_linux_name,
        build_path = build_linux_name,
        project_name = project_name,
        platform = "linux-x86_64"
    )?;

    if !no_windows {
        writeln!(
            workflow,
            include_str!("../templates/ci/release_step"),
            build_name = build_windows_name,
            build_path = build_windows_name,
            project_name = project_name,
            platform = "windows-x86_64"
        )?;
    }

    if !no_osx {
        writeln!(
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

    let mut workflow = workflow.trim_end().to_string();

    writeln!(workflow)?;

    Ok(workflow)
}
