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

    Search { pattern: String },
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
                (esprit_config::Config::default().workspace)
            );
        }

        Command::Search { pattern } => {
            let files = esprit_search::search(&pattern, ".");

            println!("Found {} matches\n", files.len());

            for file in files {
                println!("{}", file.display());
            }
        }
    }
}
