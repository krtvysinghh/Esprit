use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;
use sysinfo::System;

#[derive(Parser)]
#[command(name = "esprit", version)]
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

        #[arg(short, long)]
        content: bool,
    },

    Organize {
        folder: String,
    },
}

fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(70));

    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    pb.set_message(msg.to_string());

    pb
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Version => {
            println!("{}", esprit_core::banner());
        }

        Command::Doctor => {
            let mut s = System::new_all();

            s.refresh_all();

            println!("{}", "Esprit Doctor".green().bold());

            println!("CPU : {}", s.cpus().len());

            println!("RAM : {} MB", s.total_memory() / 1024 / 1024);

            println!("Model : {}", esprit_ai::model());
        }

        Command::Search { pattern, content } => {
            let pb = spinner("Searching");

            if content {
                let hits = esprit_search::search_contents(&pattern, ".");

                pb.finish_and_clear();

                println!("{} matches\n", hits.len());

                for (file, line, text) in hits {
                    println!(
                        "{}:{} {}",
                        file.display().to_string().cyan(),
                        line.to_string().yellow(),
                        text.trim()
                    );
                }
            } else {
                let files = esprit_search::search(&pattern, ".");

                pb.finish_and_clear();

                println!("{} files\n", files.len());

                for f in files {
                    println!("{}", f.display());
                }
            }
        }

        Command::Organize { folder } => {
            esprit_filesystem::organize(folder)?;
        }
    }

    Ok(())
}
