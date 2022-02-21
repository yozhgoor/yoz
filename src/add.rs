use anyhow::Result;

/// Add useful content to your Rust project (like licenses or CI).
#[derive(clap::Parser)]
pub struct Add {
    #[clap(long)]
    licenses: bool,
    #[clap(long)]
    ci: bool,
}

impl Add {
    pub fn run(self) -> Result<()> {
        todo!("Implement Add");
    }
}
