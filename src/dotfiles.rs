use crate::{background::Position, value_or_default, values_or_default};
use anyhow::Result;
use std::{fs, path::PathBuf};

/// Generate config files from dotfiles
#[derive(Debug, clap::Parser)]
pub struct Dotfiles {
    #[clap(long)]
    temporary_project_dir: Option<PathBuf>,
    #[clap(long)]
    editor: Option<String>,
    #[clap(long)]
    terminal: Option<String>,
    #[clap(long)]
    bg_position: Option<Position>,
    #[clap(long)]
    bg_file_path: Option<PathBuf>,
    #[clap(long)]
    fonts: Vec<String>,
    #[clap(long)]
    fonts_size: Option<u32>,
    #[clap(long)]
    bar_font_size: Option<u32>,
    #[clap(long)]
    browser: Option<String>,
    #[clap(long)]
    net_device: Option<String>,
    #[clap(long)]
    home_symbol: Option<String>,
}

#[allow(clippy::too_many_arguments)]
impl Dotfiles {
    pub fn run(
        self,
        default_temporary_project_dir: Option<PathBuf>,
        default_editor: Option<String>,
        default_terminal: Option<String>,
        default_bg_position: Option<Position>,
        default_bg_file_path: Option<PathBuf>,
        default_fonts: Vec<String>,
        default_fonts_size: Option<u32>,
        default_bar_font_size: Option<u32>,
        default_browser: Option<String>,
        default_net_device: Option<String>,
        default_home_symbol: Option<String>,
    ) -> Result<()> {
        let temporary_project_dir = value_or_default(
            self.temporary_project_dir,
            default_temporary_project_dir,
            "temporary_project_dir",
        )?;
        let editor = value_or_default(self.editor, default_editor, "editor")?;
        let terminal = value_or_default(self.terminal, default_terminal, "terminal")?;
        generate_cargo_temp(temporary_project_dir, editor, terminal)?;

        let net_device = value_or_default(self.net_device, default_net_device, "net_device")?;
        let bar_path = generate_i3status(net_device)?;

        let bg_position = value_or_default(self.bg_position, default_bg_position, "bg_position")?;
        let bg_file_path =
            value_or_default(self.bg_file_path, default_bg_file_path, "bg_file_path")?;
        let fonts = values_or_default(self.fonts, default_fonts, "fonts")?;
        let fonts_size = value_or_default(self.fonts_size, default_fonts_size, "fonts_size")?;
        let browser = value_or_default(self.browser, default_browser, "browser")?;
        let bar_font_size =
            value_or_default(self.bar_font_size, default_bar_font_size, "bar_font_size")?;
        generate_i3(
            bg_position,
            bg_file_path,
            fonts,
            fonts_size,
            browser,
            bar_path,
            bar_font_size,
        )?;

        generate_nvim()?;

        let home_symbol = value_or_default(self.home_symbol, default_home_symbol, "home_symbol")?;
        generate_starship(home_symbol)?;

        Ok(())
    }
}

fn generate_cargo_temp(
    temporary_project_dir: PathBuf,
    editor: String,
    mut terminal: String,
) -> Result<()> {
    let cargo_temp_path =
        xdg::BaseDirectories::with_prefix("cargo-temp")?.place_config_file("config.toml")?;

    terminal.push_str(" --command cargo watch -x check");

    fs::write(
        cargo_temp_path,
        format!(
            include_str!("../templates/configs/cargo-temp"),
            temporary_project_dir = temporary_project_dir
                .to_str()
                .expect("temporary_project_dir contains non UTF-8 characters"),
            editor = editor,
            subprocess_command = terminal,
        ),
    )?;

    Ok(())
}

fn generate_i3(
    bg_position: Position,
    bg_file_path: PathBuf,
    fonts: Vec<String>,
    fonts_size: u32,
    browser: String,
    bar_path: PathBuf,
    bar_font_size: u32,
) -> Result<()> {
    let i3_path = xdg::BaseDirectories::with_prefix("i3")?.place_config_file("config")?;
    let background_command = format!(
        "feh --bg-{} {}",
        bg_position,
        bg_file_path
            .to_str()
            .expect("bg_file_path contains non UTF-8 characters")
    );
    let mut it = fonts.into_iter();
    let mut fonts = it.next().expect("fonts is empty");
    for i in it {
        fonts.push_str(", ");
        fonts.push_str(&i);
    }
    let bar_position = "top";

    fs::write(
        i3_path,
        format!(
            include_str!("../templates/configs/i3"),
            background_command = background_command,
            fonts = fonts,
            fonts_size = fonts_size,
            browser = browser,
            bar_font_size = bar_font_size,
            bar_path = bar_path
                .to_str()
                .expect("bar_path contains non UTF-8 characters"),
            bar_position = bar_position,
        ),
    )?;

    Ok(())
}

fn generate_i3status(net_device: String) -> Result<PathBuf> {
    let i3status_path =
        xdg::BaseDirectories::with_prefix("i3status")?.place_config_file("config.toml")?;

    fs::write(
        i3status_path.clone(),
        format!(
            include_str!("../templates/configs/i3status"),
            net_device = net_device,
        ),
    )?;

    Ok(i3status_path)
}

fn generate_nvim() -> Result<()> {
    let nvim_path = xdg::BaseDirectories::with_prefix("nvim")?.place_config_file("init.vim")?;

    fs::copy(
        PathBuf::from("/home/yozhgoor/repos/config-files/nvim"),
        nvim_path,
    )?;

    Ok(())
}

fn generate_starship(home_symbol: String) -> Result<()> {
    let starship_path = xdg::BaseDirectories::new()?.place_config_file("starship.toml")?;

    fs::write(
        starship_path,
        format!(
            include_str!("../templates/configs/starship"),
            home_symbol = home_symbol,
        ),
    )?;

    Ok(())
}
