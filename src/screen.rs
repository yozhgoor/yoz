use anyhow::{bail, ensure, Context, Result};
use std::{collections::HashMap, fmt, process, str::FromStr};

/// Set up screens using xrandr.
///
/// If nothing is provided as argument, this will list the detected monitors.
///
/// This subcommand aims to be used with a laptop and, optionally, an external
/// screen.
#[derive(clap::Parser)]
pub struct Screen {
    /// Enable the external screen and disable the laptop screen.
    #[clap(long)]
    laptop: bool,
    /// Enable the laptop screen and disable the external screen.
    #[clap(long)]
    external: bool,
    /// Enable both screens
    #[clap(short = 'l', long)]
    all: bool,
    /// Set the refresh rate of the external monitor
    #[clap(long, default_value_t = 60)]
    rate: u8,
    /// Set the position of the external screen related to the position of the
    /// laptop screen.
    #[clap(long, default_value_t = Direction::Right)]
    direction: Direction,
}

impl Screen {
    pub fn run(self) -> Result<()> {
        let mut monitors = match fetch_monitors() {
            Ok(monitors) => monitors,
            Err(err) => bail!("cannot fetch monitors: {}", err),
        };

        if self.laptop {
            match monitors.remove(&0) {
                Some(monitor) => {
                    ensure!(
                        process::Command::new("xrandr")
                            .arg("--output")
                            .arg(&monitor.name)
                            .arg("--mode")
                            .arg(format!("{}x{}", monitor.width, monitor.height))
                            .arg("--refresh-rate")
                            .arg(monitor.rate.to_string())
                            .status()?
                            .success(),
                        "cannot enable the laptop screen"
                    );
                }
                None => bail!("laptop screen not detected"),
            }

            if let Some(external_monitor) = monitors.remove(&1) {
                ensure!(
                    process::Command::new("xrandr")
                        .arg("--output")
                        .arg(external_monitor.name)
                        .arg("--off")
                        .status()?
                        .success(),
                    "cannot disable the external screen"
                );
            }
        } else if self.external {
            match monitors.remove(&1) {
                Some(mut monitor) => {
                    monitor.rate = self.rate;
                    ensure!(
                        process::Command::new("xrandr")
                            .arg("--output")
                            .arg(&monitor.name)
                            .arg("--mode")
                            .arg(format!("{}x{}", monitor.width, monitor.height))
                            .arg("--refresh-rate")
                            .arg(monitor.rate.to_string())
                            .status()?
                            .success(),
                        "cannot enable external screen"
                    );
                }
                None => bail!("external screen not detected"),
            }
        } else if self.all {
            let laptop_monitor = match monitors.remove(&0) {
                Some(monitor) => monitor,
                None => bail!("laptop monitor not detected"),
            };
            let mut external_monitor = match monitors.remove(&1) {
                Some(monitor) => monitor,
                None => bail!("external monitor not detected"),
            };
            external_monitor.rate = self.rate;

            let direction = match self.direction {
                Direction::Left => "--left-of",
                Direction::Right => "--right-of",
            };

            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(&laptop_monitor.name)
                    .arg("--mode")
                    .arg(format!(
                        "{}x{}",
                        laptop_monitor.width, laptop_monitor.height
                    ))
                    .arg("--refresh-rate")
                    .arg(laptop_monitor.rate.to_string())
                    .status()?
                    .success(),
                "cannot enable laptop screen",
            );

            ensure!(
                process::Command::new("xrandr")
                    .arg("--output")
                    .arg(&external_monitor.name)
                    .arg(direction)
                    .arg(&laptop_monitor.name)
                    .arg("--mode")
                    .arg(format!(
                        "{}x{}",
                        laptop_monitor.width, laptop_monitor.height
                    ))
                    .arg("--refresh-rate")
                    .arg(external_monitor.rate.to_string())
                    .status()?
                    .success(),
                "cannot enable external screen",
            );
        } else {
            for monitor in monitors.values() {
                println!("{}", monitor);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Monitor {
    id: u8,
    name: String,
    width: u16,
    height: u16,
    rate: u8,
}

impl Monitor {
    fn from_xrandr_output(output: &str) -> Self {
        let id = output
            .trim()
            .chars()
            .next()
            .expect("monitor output start with the screen id")
            .to_digit(10)
            .expect("screen id is an integer")
            .try_into()
            .expect("id cannot be higher than 1");

        let mut split = output
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let name = split
            .pop()
            .expect("xrandr's output cannot be empty")
            .to_string();

        let resolution = split
            .pop()
            .expect("the resolution come before the monitor name")
            .split('x')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.split('/')
                    .next()
                    .expect("xrandr's output contains the resolution")
                    .parse::<u16>()
                    .expect("Resolution is an integer")
            })
            .collect::<Vec<u16>>();

        Self {
            id,
            name,
            width: resolution[0],
            height: resolution[1],
            rate: 60,
        }
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} {}x{} {}Hz",
            self.id, self.name, self.width, self.height, self.rate
        )
    }
}

enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.to_lowercase();

        let direction = match s.as_str() {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => bail!("Cannot parse direction from {}", s),
        };

        Ok(direction)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

fn fetch_monitors() -> Result<HashMap<u8, Monitor>> {
    let output = process::Command::new("xrandr")
        .arg("--listmonitors")
        .output()
        .context("`xrandr --listmonitors` failed")?;

    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => bail!("cannot parse `xrandr` output: {}", err),
    };
    log::debug!("stdout:\n{}", stdout);

    let mut lines = stdout.lines();

    let monitors_num = lines
        .next()
        .expect("`xrandr --listmonitors` always print the number of monitors")
        .strip_prefix("Monitors: ")
        .expect("`xrandr --listmonitors` start with `Monitors:`")
        .parse::<u8>()
        .expect("`xrandr --listmonitors` print the number of monitors as an integer");
    log::info!("numbers of monitors: {}", monitors_num);

    match monitors_num {
        0 => bail!("no monitor detected"),
        1 => {
            let monitor = Monitor::from_xrandr_output(lines.next().expect("one monitor detected"));
            Ok(HashMap::from([(monitor.id, monitor)]))
        }
        2 => {
            let first_monitor =
                Monitor::from_xrandr_output(lines.next().expect("two monitor detected"));
            let second_monitor =
                Monitor::from_xrandr_output(lines.next().expect("two monitor detected"));

            Ok(HashMap::from([
                (first_monitor.id, first_monitor),
                (second_monitor.id, second_monitor),
            ]))
        }
        _ => {
            unimplemented!("more than two monitors detected");
        }
    }
}
