use clap::{Parser, Subcommand};
mod health;

#[derive(Parser)]
#[command(name = "esprit", version, about = "Local AI Knowledge Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify local dependencies and environment readiness
    Doctor,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Doctor => {
            println!("🩺 Running Esprit Environment Check...");
            let ollama_status = if health::is_ollama_running() { "✅ OK" } else { "❌ OFFLINE" };
            println!("Ollama Daemon (127.0.0.1:11434) : {}", ollama_status);
            println!("Tantivy Index Directory         : ⚠️ PENDING");
            println!("SQLite Vector Store             : ⚠️ PENDING");
        }
    }
}
