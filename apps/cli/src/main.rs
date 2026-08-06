use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "esprit", version, about = "Esprit AI Operating Layer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Version,
    Doctor,
    Search { query: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Version => {
            println!("{}", esprit_core::banner());
        }

        Command::Doctor => {
            println!("Platform : {:?}", esprit_platform::current());
            println!("Model    : {}", esprit_ai::model());
            println!(
                "Workspace: {:?}",
                esprit_config::Config::default().workspace
            );
        }

        Command::Search { query } => {
            for file in esprit_search::find(&query, ".") {
                println!("{}", file.display());
            }
        }
    }
}
