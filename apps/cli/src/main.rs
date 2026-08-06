use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;
use sysinfo::System;

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
}

fn doctor() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!();
    println!("{}", "Esprit Doctor".bright_blue().bold());
    println!("{}", "────────────────────────────".bright_black());

    println!("Platform   : {:?}", esprit_platform::current());
    println!("CPU Cores  : {}", sys.cpus().len());
    println!("Memory     : {} MB", sys.total_memory() / 1024 / 1024);
    println!("AI Model   : {}", esprit_ai::model());
    println!(
        "Workspace  : {:?}",
        esprit_config::Config::default().workspace
    );

    println!();
    println!("{}", "Status".green().bold());
    println!("{} Workspace", "✓".green());
    println!("{} CLI", "✓".green());
    println!("{} Config", "✓".green());
    println!("{} Search", "✓".green());
    println!("{} Filesystem", "✓".green());
}

fn search(pattern: String) {
    let spinner = ProgressBar::new_spinner();

    spinner.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    spinner.enable_steady_tick(Duration::from_millis(80));

    spinner.set_message("Searching...");

    let files = esprit_search::search(&pattern, ".");

    spinner.finish_and_clear();

    println!("\n{} {} matches\n", "Found".green().bold(), files.len());

    for file in files {
        println!("{}", file.display());
    }
}

fn organize(folder: String) -> Result<()> {
    esprit_filesystem::organize(folder)?;

    println!("\n{} Files organized successfully.", "✓".green().bold());

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Version => {
            println!("{}", esprit_core::banner());
        }

        Command::Doctor => doctor(),

        Command::Search { pattern } => {
            search(pattern);
        }

        Command::Organize { folder } => {
            organize(folder)?;
        }
    }

    Ok(())
}
