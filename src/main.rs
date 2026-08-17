mod manifest;

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use manifest::{Manifest, Project};
use std::path::Path;

#[derive(Parser)]
#[command(name = "plumbum", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Bootstrap a new multi-place project
    Init {
        /// Project name (defaults to the current directory name)
        name: Option<String>,

        /// pesde scope/namespace
        #[arg(long)]
        scope: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { name, scope } => init(name, scope),
    }
}

fn init(name: Option<String>, scope: Option<String>) -> anyhow::Result<()> {
    if Path::new("plumbum.toml").exists() {
        bail!("this directory is already a Plumbum project (plumbum.toml exists)");
    }

    for dir in [
        ".plumbum/configs/places",
        ".plumbum/configs/packages",
        ".plumbum/configs/scripts",
        "places",
        "packages",
        "scripts",
    ] {
        std::fs::create_dir_all(dir).with_context(|| format!("creating directory {dir}"))?;
    }

    let manifest = Manifest {
        project: Project {
            name: name.unwrap_or_else(|| "unnamed".to_string()),
            scope: scope.unwrap_or_else(|| "unknown".to_string()),
            version: "0.1.0".to_string(),
        },
    };
    let toml = toml::to_string(&manifest).expect("serialize manifest");
    std::fs::write("plumbum.toml", toml).expect("write plumbum.toml");

    println!("Initialized Plumbum project");

    Ok(())
}
