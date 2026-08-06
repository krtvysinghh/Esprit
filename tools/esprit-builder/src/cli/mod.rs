use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "esprit-builder", version, about = "Esprit Project Generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Doctor,
    New { kind: String, name: String },
}
