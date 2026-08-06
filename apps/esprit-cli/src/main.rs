use anyhow::Result;
use clap::{Parser, Subcommand};
use esprit_search::{SearchEngine, SearchOptions};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "esprit", version, about = "Esprit AI Operating Layer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Doctor,
    Version,
    Search {
        pattern: String,

        #[arg(long)]
        regex: bool,
    },
}

fn doctor() -> Result<()> {
    println!("{}", esprit_core::banner());

    let cfg = esprit_config::Config::load()?;

    println!("Platform : {:?}", esprit_platform::current());
    println!("AI Model : {}", cfg.ai_model);
    println!("Workspace: {}", cfg.workspace.display());

    Ok(())
}

fn version() {
    println!("{}", esprit_core::VERSION);
}

fn search(pattern: String, regex: bool) -> Result<()> {
    let results = SearchEngine::run(SearchOptions { root: PathBuf::from("."), pattern, regex })?;

    println!("{} match(es)\n", results.len());

    for result in results {
        println!("{}", result.path.display());
    }

    Ok(())
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Doctor => doctor()?,
        Command::Version => version(),
        Command::Search { pattern, regex } => search(pattern, regex)?,
    }

    Ok(())
}
