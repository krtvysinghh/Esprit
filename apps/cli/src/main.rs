use anyhow::Result;
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

    Organize { folder: String },

    Stats { folder: String },
}

fn main() -> Result<()> {
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

        Command::Search { pattern } => {
            let files = esprit_search::search(&pattern, ".");

            println!("{} matches\n", files.len());

            for file in files {
                println!("{}", file.display());
            }
        }

        Command::Organize { folder } => {
            esprit_filesystem::organize(folder)?;
        }

        Command::Stats { folder } => {
            let stats = esprit_filesystem::stats::FolderStats::scan(folder)?;

            println!();
            println!("Files       : {}", stats.files);
            println!("Directories : {}", stats.directories);
            println!("Size        : {} bytes", stats.bytes);

            println!("\nExtensions");

            let mut exts: Vec<_> = stats.extensions.into_iter().collect();

            exts.sort_by(|a, b| b.1.cmp(&a.1));

            for (ext, count) in exts {
                println!("{:>8} : {}", ext, count);
            }
        }
    }

    Ok(())
}
