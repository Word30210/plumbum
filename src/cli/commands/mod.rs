use std::path::PathBuf;

mod init;

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Bootstrap a new multi-place project
    Init(init::InitCommand),
}

impl Subcommand {
    pub fn run(self, cwd: PathBuf) -> anyhow::Result<()> {
        match self {
            Subcommand::Init(init) => init.run(&cwd),
        }
    }
}
