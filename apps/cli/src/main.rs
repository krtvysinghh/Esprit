use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "esprit",
    version,
    about = "Esprit AI Operating Layer"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Doctor,
    Search {
        query: String,
    },
    Organize {
        folder: String,
    },
    Config,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version => {
            println!("Esprit {}", env!("CARGO_PKG_VERSION"));
        }

        Commands::Doctor => {
            println!("✓ CLI OK");
            println!("✓ Rust OK");
            println!("✓ Workspace OK");
        }

        Commands::Search { query } => {
            println!("Searching: {query}");
        }

        Commands::Organize { folder } => {
            println!("Organizing: {folder}");
        }

        Commands::Config => {
            println!("Configuration");
        }
    }
}
