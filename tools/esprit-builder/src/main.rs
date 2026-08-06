use clap::Parser;

use esprit_builder::{
    cli::{Cli, Command},
    generator::{Generator, crate_generator::CrateGenerator, workspace::WorkspaceGenerator},
    logging,
};

fn main() -> anyhow::Result<()> {
    logging::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            WorkspaceGenerator.generate()?;
        }

        Command::Doctor => {
            println!("OK");
        }

        Command::New { kind, name } => match kind.as_str() {
            "crate" => {
                CrateGenerator { name }.generate()?;
            }

            _ => println!("Unknown generator"),
        },
    }

    Ok(())
}
