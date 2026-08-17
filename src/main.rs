mod manifest;

use manifest::{Manifest, Project};

use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { name, scope } => {
            let manifest = Manifest {
                project: Project {
                    name: name.unwrap_or_else(|| "unnamed".to_string()),
                    scope: scope.unwrap_or_else(|| "unknown".to_string()),
                    version: "0.1.0".to_string(),
                },
            };

            let toml = toml::to_string(&manifest).expect("serialize manifest");
            std::fs::write("plumbum.toml", toml).expect("write plumbum.toml");
            println!("created plumbum.toml");
        }
    }
}
