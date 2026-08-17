use clap::{Parser,Subcommand};

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
            println!("init: name={name:?}, scope={scope:?}")
        }
    }
}
