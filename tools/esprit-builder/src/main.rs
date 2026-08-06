use clap::Parser;

use esprit_builder::{
    cli::{Cli, Command},
    generator::{Generator, workspace::WorkspaceGenerator},
    logging,
};

fn main() -> anyhow::Result<()> {
    logging::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            WorkspaceGenerator.generate()?;
            println!("Workspace initialized.");
        }

        Command::Doctor => {
            println!("Diagnostics OK");
        }

        Command::New { kind, name } => {
            println!("{kind}: {name}");
        }
    }

    Ok(())
}
