use clap::{Parser, Subcommand};
mod health;
mod storage;

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
            let ollama = if health::is_ollama_running() { "✅ OK" } else { "❌ OFFLINE" };
            let tantivy = if storage::check_tantivy_dir() { "✅ OK" } else { "❌ FAILED" };
            let sqlite = if storage::check_sqlite_store() { "✅ OK" } else { "❌ FAILED" };
            
            println!("Ollama Daemon (127.0.0.1:11434) : {}", ollama);
            println!("Tantivy Index Directory         : {}", tantivy);
            println!("SQLite Vector Store             : {}", sqlite);
        }
    }
}
