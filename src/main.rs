use clap::{Parser, Subcommand};
use std::path::Path;
mod health;
mod storage;
mod db;
mod index;

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
    /// Initialize the database and index structures
    Init,
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
        Commands::Init => {
            println!("🚀 Initializing Esprit workspace...");
            if storage::check_sqlite_store() {
                let db_path = Path::new(".esprit/db/esprit.db");
                match db::init_db(db_path) {
                    Ok(_) => println!("✅ SQLite database initialized at {:?}", db_path),
                    Err(e) => println!("❌ Database initialization failed: {}", e),
                }
            } else {
                println!("❌ Failed to create db directory.");
            }
            
            if storage::check_tantivy_dir() {
                let index_path = Path::new(".esprit/index");
                match index::init_index(index_path) {
                    Ok(_) => println!("✅ Tantivy index initialized at {:?}", index_path),
                    Err(e) => println!("❌ Tantivy initialization failed: {}", e),
                }
            } else {
                println!("❌ Failed to create index directory.");
            }
        }
    }
}
