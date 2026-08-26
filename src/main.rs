mod cli;

use anyhow::Context;
use clap::Parser;
use plumbum::display_err;

#[derive(Parser)]
#[command(name = "plumbum", version, about)]
struct Cli {
    #[command(subcommand)]
    subcommand: cli::commands::Subcommand,
}

fn run() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().context("failed to get current working directory")?;

    let cli = Cli::parse();
    cli.subcommand.run(cwd)
}

fn main() {
    let res = run();
    let is_err = res.is_err();
    display_err(res, "");
    if is_err {
        std::process::exit(1);
    }
}
