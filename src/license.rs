use anyhow::Result;
use chrono::Datelike;
use std::{fs, path::Path};

pub fn add_licenses(project_dir_path: &Path, full_name: String) -> Result<()> {
    let year = chrono::Local::now().date().year();

    fs::write(
        project_dir_path.join("LICENSE.MIT"),
        format!(
            include_str!("../templates/licenses/mit"),
            year = year,
            full_name = full_name
        ),
    )?;

    fs::write(
        project_dir_path.join("LICENSE.Apache-2.0"),
        format!(
            include_str!("../templates/licenses/apache"),
            year = year,
            full_name = full_name,
        ),
    )?;

    Ok(())
}
