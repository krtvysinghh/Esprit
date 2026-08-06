use clap::Parser;
use esprit_builder::{
    cli::{Cli, Command},
    logging,
};

fn main() {
    logging::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            println!("Initializing workspace...");
        }

        Command::Doctor => {
            println!("Running diagnostics...");
        }

        Command::New { kind, name } => {
            println!("Create {} {}", kind, name);
        }
    }
}
