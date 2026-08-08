use anyhow::Result;
use clap::{Parser, Subcommand};
use esprit_agents::{run, Agent};
use esprit_ai::Ai;
use esprit_config::Config;
use esprit_filesystem::stats::FolderStats;
use esprit_filesystem::{duplicates, organize};
use esprit_index::{all_files, index, rebuild_search_index, search};
use esprit_platform::{doctor, watch};

#[derive(Parser)]
#[command(name = "esprit", version, about = "Esprit AI Operating Layer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Doctor,
    Version,
    Config,

    Search {
        pattern: String,

        #[arg(long)]
        regex: bool,
    },

    Stats {
        folder: String,
    },

    Organize {
        folder: String,
    },

    Duplicates {
        folder: String,
    },

    Index {
        folder: String,
    },

    Rebuild,

    Db,

    Watch {
        folder: String,
    },

    Ask {
        prompt: String,
    },

    Agent {
        agent: String,
        prompt: String,
    },

    Workflow {
        workflow: String,
        prompt: String,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Version => {
            println!("{}", esprit_core::banner());
        }

        Commands::Doctor => {
            let report = doctor();

            println!("{}", esprit_core::banner());
            println!("Operating System : {}", report.os);
            println!("Kernel           : {}", report.kernel);
            println!("Hostname         : {}", report.hostname);
            println!("CPU              : {}", report.cpu);
            println!("CPU Cores        : {}", report.cpu_cores);
            println!("Memory (GB)      : {:.2}", report.ram_gb);

            println!();
            println!("Development Tools");
            println!("Rust   : {}", if report.rust { "✓" } else { "✗" });
            println!("Cargo  : {}", if report.cargo { "✓" } else { "✗" });
            println!("Git    : {}", if report.git { "✓" } else { "✗" });
            println!("Ollama : {}", if report.ollama { "✓" } else { "✗" });
        }

        Commands::Config => {
            let cfg = Config::load()?;

            println!("AI Model  : {}", cfg.ai_model);
            println!("Workspace : {}", cfg.workspace.display());
            println!("Threads   : {}", cfg.threads);
            println!("Color     : {}", cfg.color);
        }

        Commands::Search { pattern, .. } => {
            let results = search(&pattern)?;

            println!("Found {} matches\n", results.len());

            for path in results {
                println!("{path}");
            }
        }
        Commands::Stats { folder } => {
            let stats = FolderStats::scan(folder)?;

            println!("Files       : {}", stats.files);
            println!("Directories : {}", stats.directories);
            println!("Size        : {} bytes", stats.bytes);

            let mut exts: Vec<_> = stats.extensions.into_iter().collect();
            exts.sort_by_key(|e| std::cmp::Reverse(e.1));

            println!("\nExtensions:");

            for (ext, count) in exts {
                println!("{:>8} {}", ext, count);
            }
        }

        Commands::Organize { folder } => {
            organize(folder)?;
            println!("Done.");
        }

        Commands::Duplicates { folder } => {
            let groups = duplicates(folder)?;

            if groups.is_empty() {
                println!("No duplicates found.");
            } else {
                println!("Duplicate groups: {}\n", groups.len());

                for (i, group) in groups.iter().enumerate() {
                    println!("Group {}", i + 1);

                    for file in group {
                        println!("  {}", file.display());
                    }

                    println!();
                }
            }
        }

        Commands::Index { folder } => {
            let files = index(folder)?;
            println!("Indexed {} files.", files.len());
        }

        Commands::Rebuild => {
            rebuild_search_index()?;
        }

        Commands::Db => {
            let files = all_files()?;

            println!("Database contains {} files.\n", files.len());

            for file in files {
                println!("{:>10} {}", file.size, file.path.display());
            }
        }

        Commands::Watch { folder } => {
            watch(folder)?;
        }

        Commands::Agent { agent, prompt } => {
            let agent = match agent.as_str() {
                "chat" => Agent::Chat,
                "code" => Agent::Code,
                "search" => Agent::Search,
                _ => anyhow::bail!("unknown agent"),
            };

            println!("{}", run(agent, &prompt)?);
        }

        Commands::Workflow { workflow, prompt } => {
            let out = match workflow.as_str() {
                "explain" => esprit_workflows::explain(&prompt)?,
                "review" => esprit_workflows::code_review(&prompt)?,
                "search" => esprit_workflows::project_search(&prompt)?,
                _ => anyhow::bail!("unknown workflow"),
            };

            println!("{out}");
        }

        Commands::Ask { prompt } => {
            let ai = Ai::new("qwen3:1.7b");
            ai.health()?;
            println!("{}", esprit_rag::ask(&prompt)?);
        }
    }

    Ok(())
}
