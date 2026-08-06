use anyhow::Result;
use clap::{Parser, Subcommand};
use esprit_search::{SearchEngine, SearchOptions};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "esprit", version, about = "Esprit AI Operating Layer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Doctor,
    Version,
    Search {
        pattern: String,

        #[arg(long)]
        regex: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Version => {
            println!("{}", esprit_core::banner());
        }

        Commands::Doctor => {
            let report = esprit_platform::doctor();

            println!("{}", esprit_core::banner());

            println!("Operating System : {}", report.os);
            println!("Kernel           : {}", report.kernel);
            println!("Hostname         : {}", report.hostname);
            println!("CPU              : {}", report.cpu);
            println!("CPU Cores        : {}", report.cpu_cores);
            println!("Memory (GB)      : {:.2}", report.ram_gb);

            println!();
            println!("Development Tools");
            println!("Rust   : {}", if report.rust { "✓" } else { "✗" });
            println!("Cargo  : {}", if report.cargo { "✓" } else { "✗" });
            println!("Git    : {}", if report.git { "✓" } else { "✗" });
            println!("Ollama : {}", if report.ollama { "✓" } else { "✗" });
        }

        Commands::Search { pattern, regex } => {
            let results =
                SearchEngine::run(SearchOptions { root: PathBuf::from("."), pattern, regex })?;

            println!("Found {} matches\n", results.len());

            for result in results {
                println!("{}", result.path.display());
            }
        }
    }

    Ok(())
}
