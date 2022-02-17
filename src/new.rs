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
}

impl New {
    pub fn run(self) -> anyhow::Result<()> {
        let working_dir = crate::set_working_dir(self.path)?;

        let project_dir_path = working_dir.join(self.name);
        std::fs::create_dir(&project_dir_path)?;

        if !self.no_license {
            add_licenses(&project_dir_path, self.full_name)?;
        }

        if !self.no_ci {
            add_ci(&project_dir_path, &self.name)?;
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

use serde::{ Serialize, Deserialize};

fn add_ci(project_dir_path: &std::path::Path) -> anyhow::Result<()> {
    let workflows_dir = project_dir_path.join(".github/workflows");
    std::fs::create_dir_all(workflows_dir)?;

    let workflows = Workflows::bin(&self.name);

    let reader = std::fs::File::open("main.yml")?;
    let de = serde_yaml::Deserializer::from_reader(reader);
    let value = serde_yaml::Value::deserialize(de)?;
    println!("{:?}", value);

    Ok(())
}

#[derive(Debug)]
struct Workflows {
    main_workflow: MainWorkflow,
    pr_workflow: PrWorkflow,
    release_workflow: Option<ReleaseWorkflow>,
}

impl Workflows {
    fn bin(name: &std::path::Path) -> Self {
        let tests = vec![
            Step {
                name: Some("Checkout source".to_string()),
                id: None,
                run: None,
                uses: Some("actions/checkout@v2".to_string()),
                with: None,
                with_run: None,
                env: None,
            },
            Step {
                name: None,
                id: None,
                run: None,
                uses: Some("Swatinem/rust-cache@v1".to_string()),
                with: None,
                with_run: None,
                env: None,
            },
            Step {
                name: Some("cargo test".to_string()),
                id: None,
                run: None,
                uses: Some("actions-rs/cargo@v1".to_string()),
                with: Some(With {
                    targets: None,
                    command: Some("test".to_string()),
                    args: Some("--workspace".to_string()),
                    token: None,
                    name: None,
                    path: None,
                    files: None,
                }),
                with_run: None,
                env: None,
            }
        ];

        let main_workflow = MainWorkflow {
            name: "main".to_string(),
            on: On {
                push: Some(Push {
                    branches: Some(vec!["main".to_string()]),
                    tags: None,
                }),
                pull_request: None,
                schedule: Some(Schedule {
                    cron: "0 0 1 * *".to_string()
                }),
            },
            env: CargoEnv {
                cargo_term_color: "always".to_string(),
            },
            jobs: MainJobs {
                test: Test {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: tests,
                },
                test_macos: Test {
                    runs_on: "macos-latest".to_string(),
                    steps: tests,
                },
                test_windows: Test {
                    runs_on: "windows-latest".to_string(),
                    steps: tests,
                }
            }
        };

        let pr_workflow = PrWorkflow {
            name: "PR".to_string(),
            on: On {
                push: None,
                pull_request: Some(PullRequest {
                    branches: vec!["main".to_string()],
                }),
                schedule: None,
            },
            env: CargoEnv {
                cargo_term_color: "always".to_string(),
            },
            jobs: PrJobs {
                test_and_lint: Test {

                },
                test_windows: Test {
                     runs_on: "windows-latest".to_string(),
                     steps: tests,
                },
                test_osx: Test {
                     runs_on: "macos-latest".to_string(),
                     steps: tests,
                },
             },
        };

        Self {
            main_workflow,
            pr_workflow,
            release_workflow,
        }
    }

    fn lib(name: &std::path::Path) -> Self {
        todo!("lib workflows");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MainWorkflow {
    name: String,
    on: On,
    env: CargoEnv,
    jobs: MainJobs,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrWorkflow {
    name: String,
    on: On,
    env: CargoEnv,
    jobs: PrJobs,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseWorkflow {
    name: String,
    on: On,
    jobs: ReleaseJobs,
}

#[derive(Debug, Serialize, Deserialize)]
struct On {
    push: Option<Push>,
    pull_request: Option<PullRequest>,
    schedule: Option<Schedule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {
    branches: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
struct Push {
    branches: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schedule {
    cron: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoEnv {
    #[serde(rename = "CARGO_TERM_COLOR")]
    cargo_term_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GithubEnv {
    #[serde(rename = "GITHUB_TOKEN")]
    github_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MainJobs {
    test: Test,
    #[serde(rename = "test-windows")]
    test_windows: Test,
    #[serde(rename = "test-macos")]
    test_macos: Test,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrJobs {
    test_and_lint: Test,
    #[serde(rename = "test-windows")]
    test_windows: Test,
    #[serde(rename = "test-osx")]
    test_osx: Test,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseJobs {
    #[serde(rename = "build-linux")]
    build_linux: Test,
    #[serde(rename = "build-osx")]
    build_osx: Test,
    #[serde(rename = "build_windows")]
    build_windows: Test,
    #[serde(rename = "release")]
    release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    #[serde(rename = "runs-on")]
    runs_on: String,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    needs: Vec<String>,
    #[serde(rename = "runs-on")]
    runs_on: String,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Step {
    name: Option<String>,
    id: Option<String>,
    run: Option<String>,
    uses: Option<String>,
    with: Option<With>,
    #[serde(rename = "run")]
    with_run: Option<String>,
    env: Option<GithubEnv>,
}

#[derive(Debug, Serialize, Deserialize)]
struct With {
    targets: Option<String>,
    command: Option<String>,
    args: Option<String>,
    token: Option<String>,
    name: Option<String>,
    path: Option<String>,
    files: Option<String>,
}
