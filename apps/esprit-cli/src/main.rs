use anyhow::Result;
use clap::{Parser, Subcommand};
use esprit_config::Config;
use esprit_filesystem::stats::FolderStats;
use esprit_filesystem::{duplicates, organize};
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

    Config,

    Search {
        pattern: String,

        #[arg(long)]
        regex: bool,
    },

    Stats {
        folder: String,
    },

    Organize {
        folder: String,
    },

    Duplicates {
        folder: String,
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

        Commands::Config => {
            let cfg = Config::load()?;

            println!("AI Model  : {}", cfg.ai_model);
            println!("Workspace : {}", cfg.workspace.display());
            println!("Threads   : {}", cfg.threads);
            println!("Color     : {}", cfg.color);
        }

        Commands::Search { pattern, regex } => {
            let results =
                SearchEngine::run(SearchOptions { root: PathBuf::from("."), pattern, regex })?;

            println!("Found {} matches\n", results.len());

            for result in results {
                println!("{}", result.path.display());
            }
        }

        Commands::Stats { folder } => {
            let stats = FolderStats::scan(folder)?;

            println!("Files       : {}", stats.files);
            println!("Directories : {}", stats.directories);
            println!("Size        : {} bytes", stats.bytes);

            let mut exts: Vec<_> = stats.extensions.into_iter().collect();
            exts.sort_by(|a, b| b.1.cmp(&a.1));

            println!("\nExtensions:");

            for (ext, count) in exts {
                println!("{:>8} {}", ext, count);
            }
        }

        Commands::Organize { folder } => {
            organize(folder)?;
            println!("Done.");
        }

        Commands::Duplicates { folder } => {
            let groups = duplicates(folder)?;

            if groups.is_empty() {
                println!("No duplicates found.");
            } else {
                println!("Duplicate groups: {}\n", groups.len());

                for (i, group) in groups.iter().enumerate() {
                    println!("Group {}", i + 1);

                    for file in group {
                        println!("  {}", file.display());
                    }

                    println!();
                }
            }
        }
    }

    Ok(())
}
