use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Instant;

// ── helpers ──────────────────────────────────────────────────────────────────

fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

#[allow(dead_code)]
fn bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::with_template(
            "  {msg:.bold}  [{bar:40.cyan/black}]  {pos}/{len}  {elapsed}",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    pb.set_message(msg.to_string());
    pb
}

fn ok(msg: &str) {
    println!("  {} {}", "✓".green().bold(), msg);
}

fn fail(msg: &str) {
    println!("  {} {}", "✗".red().bold(), msg);
}

fn warn(msg: &str) {
    println!("  {} {}", "⚠".yellow().bold(), msg);
}

fn section(title: &str) {
    println!("\n  {}", title.bold().underline());
}

fn divider() {
    println!("  {}", "─".repeat(52).dimmed());
}

fn kv(key: &str, val: &str) {
    println!("  {:<20} {}", key.dimmed(), val.bold());
}

fn elapsed(start: Instant) -> String {
    let ms = start.elapsed().as_millis();
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.1}s", start.elapsed().as_secs_f64())
    }
}

// ── CLI definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "esprit",
    version,
    about = "Esprit — AI workspace & operating layer",
    long_about = None,
    propagate_version = true,
)]
struct Cli {
    /// Enable verbose logging (RUST_LOG=debug).
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system health and tool availability.
    Doctor,

    /// Developer Diary notes linked to current git branch
    Diary {
        /// Note content to save (if empty, lists existing notes)
        note: Option<String>,
    },

    /// Terminal TUI Dashboard for metrics
    Dashboard,

    /// Output the architectural graph as Mermaid JS
    Graph,

    /// Output project dependencies extracted from index
    Deps,

    /// Find source files missing test files
    TestGaps,

    /// Print Esprit version and build info.
    Version,

    /// Show and manage Esprit configuration.
    Config {
        /// Set AI model (e.g. llama3.2).
        #[arg(long)]
        set_model: Option<String>,
    },

    /// Search the indexed project (keyword or regex).
    Search {
        /// Pattern to search for.
        pattern: String,

        /// Treat pattern as a regular expression.
        #[arg(long, short)]
        regex: bool,

        /// Search across all known workspaces globally
        #[arg(long)]
        all: bool,

    },

    /// Show filesystem statistics for a folder.
    Stats {
        /// Target folder to analyse.
        folder: String,
    },

    /// Organise files in a folder into extension subdirectories.
    Organize {
        /// Folder to organise.
        folder: String,

        /// Preview changes without moving files.
        #[arg(long)]
        dry_run: bool,
    },

    /// Find duplicate files by content hash.
    Duplicates {
        /// Folder to scan.
        folder: String,
    },

    /// Index a folder into the search database.
    Index {
        /// Root folder to index.
        folder: String,
    },

    /// Rebuild the full-text search index from the current database.
    Rebuild,

    /// List all files in the index database.
    Db,

    /// Show a summary of the current index.
    IndexStats,

    /// Watch a folder for changes and keep the index up to date.
    Watch {
        /// Folder to watch.
        folder: String,
    },

    /// Ask the AI a question about your indexed project.
    Ask {
        /// Your question.
        prompt: String,

        /// Show which source files were used to answer.
        #[arg(long, short)]
        sources: bool,
    },

    /// Run a named AI agent.
    Agent {
        /// Agent name: chat | code | search
        agent: String,
        /// Prompt.
        prompt: String,
    },

    /// Run a named workflow.
    Workflow {
        /// Workflow name: explain | review | search
        workflow: String,
        /// Prompt.
        prompt: String,
    },

    /// Clear conversation memory.
    MemoryClear,

    /// Show conversation memory stats.
    MemoryStats,

    /// Download default models and set up Esprit for first use.
    Init {
        /// Also download the embedding model for semantic search.
        #[arg(long)]
        with_embeddings: bool,
    },

    /// Manage local AI models.
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },
}

// ── Model sub-command ─────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum ModelAction {
    /// List all known models and their installation status.
    List,
    /// Download a model by ID (e.g. `qwen3:0.6b`).
    Pull {
        /// Model ID from `esprit model list`.
        id: String,
    },
    /// Remove an installed model to free disk space.
    Remove { id: String },
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        std::env::set_var("RUST_LOG", "esprit=debug,warn");
    }
    esprit_telemetry::init()?;

    match cli.command {
        // ── version ──────────────────────────────────────────────────────────
        Commands::Version => {
            println!("{}", esprit_core::banner().cyan().bold());
        }

        // ── doctor ───────────────────────────────────────────────────────────
        Commands::Doctor => {
            println!("{}", esprit_core::banner().cyan().bold());
            let sp = spinner("Gathering system information…");
            let report = esprit_platform::doctor();
            sp.finish_and_clear();

            section("System");
            divider();
            kv("OS", &report.os);
            kv("Kernel", &report.kernel);
            kv("Hostname", &report.hostname);
            kv("CPU", &report.cpu);
            kv("CPU Cores", &report.cpu_cores.to_string());
            kv("RAM", &format!("{:.1} GB", report.ram_gb));

            section("AI & Inference Engine");
            divider();
            #[cfg(target_os = "macos")]
            kv(
                "Inference Backend",
                "Embedded llama.cpp (Apple Silicon Metal)",
            );
            #[cfg(not(target_os = "macos"))]
            kv(
                "Inference Backend",
                "Embedded llama.cpp (Multi-threaded CPU)",
            );

            if let Ok(models) = esprit_models::list_status() {
                let installed: Vec<_> = models.into_iter().filter(|(_, inst)| *inst).collect();
                if installed.is_empty() {
                    println!(
                        "  {} No models installed yet — run: {}",
                        "⚠".yellow().bold(),
                        "esprit init".bold().cyan()
                    );
                } else {
                    for (entry, _) in installed {
                        println!(
                            "  {} {:<14} {}",
                            "✓".green().bold(),
                            entry.id.bold(),
                            entry.display.dimmed()
                        );
                    }
                }
            }

            section("Developer Tools");
            divider();
            let chk = |ok: bool, name: &str, ver: Option<&str>| {
                if ok {
                    if let Some(v) = ver {
                        println!(
                            "  {} {:<10} {}",
                            "✓".green().bold(),
                            name.bold(),
                            v.dimmed()
                        );
                    } else {
                        println!("  {} {}", "✓".green().bold(), name.bold());
                    }
                } else {
                    println!(
                        "  {} {:<10} {}",
                        "○".dimmed(),
                        name.dimmed(),
                        "optional (not found)".dimmed()
                    );
                }
            };
            chk(report.git, "Git", report.git_version.as_deref());
            chk(report.rust, "Rust", report.rust_version.as_deref());
            chk(report.cargo, "Cargo", None);
            chk(report.ollama, "Ollama", report.ollama_version.as_deref());

            section("Index");
            divider();
            match esprit_index::index_stats() {
                Ok(s) => {
                    kv("Files indexed", &s.file_count.to_string());
                    kv(
                        "Total size",
                        &format!("{:.1} MB", s.total_bytes as f64 / 1_048_576.0),
                    );
                }
                Err(_) => {
                    warn("No index yet — run: esprit index <folder>");
                }
            }
            println!();
        }

        // ── config ───────────────────────────────────────────────────────────
        Commands::Config { set_model } => {
            let mut cfg = esprit_config::Config::load()?;

            if let Some(model) = set_model {
                cfg.set_model(&model)?;
                ok(&format!("AI model set to {}", model.bold()));
                return Ok(());
            }

            section("Esprit Configuration");
            divider();
            kv("AI Model", &cfg.ai_model);
            kv("Ollama URL", &cfg.ollama_url);
            kv("Workspace", &cfg.workspace.display().to_string());
            kv("Threads", &cfg.threads.to_string());
            kv("Color", if cfg.color { "enabled" } else { "disabled" });
            kv(
                "Context chars/file",
                &cfg.context_chars_per_file.to_string(),
            );
            kv("Max context files", &cfg.max_context_files.to_string());
            println!();
        }

        // ── search ───────────────────────────────────────────────────────────
        Commands::Search { pattern, regex: _, all } => {
            let sp = spinner(&format!("Searching for \"{}\"…", pattern.bold()));
            let t = Instant::now();
            let results = if all {
                esprit_index::search_all_workspaces(&pattern)
            } else {
                esprit_index::search(&pattern)
            };
            sp.finish_and_clear();

            match results {
                Err(e) => {
                    fail(&format!("Search failed: {e}"));
                    warn("Run `esprit rebuild` to build the search index first.");
                }
                Ok(results) => {
                    if results.is_empty() {
                        println!(
                            "\n  {} No results for \"{}\"\n",
                            "○".dimmed(),
                            pattern.yellow()
                        );
                    } else {
                        println!(
                            "\n  {} Found {} match{} for \"{}\"  {}\n",
                            "●".cyan(),
                            results.len().to_string().bold(),
                            if results.len() == 1 { "" } else { "es" },
                            pattern.yellow(),
                            elapsed(t).dimmed(),
                        );
                        for path in &results {
                            println!("  {} {}", "│".dimmed(), path.cyan());
                        }
                        println!();
                    }
                }
            }
        }

        // ── stats ────────────────────────────────────────────────────────────
        Commands::Stats { folder } => {
            let sp = spinner(&format!("Scanning {}…", folder.bold()));
            let t = Instant::now();
            let stats = esprit_filesystem::stats::FolderStats::scan(&folder)?;
            sp.finish_and_clear();

            section(&format!("Stats — {folder}"));
            divider();
            kv("Files", &stats.files.to_string());
            kv("Directories", &stats.directories.to_string());
            kv(
                "Total size",
                &format!("{:.1} MB", stats.bytes as f64 / 1_048_576.0),
            );

            let mut exts: Vec<_> = stats.extensions.into_iter().collect();
            exts.sort_by_key(|(_, n)| std::cmp::Reverse(*n));

            println!();
            println!("  {}", "Extensions:".bold());
            for (ext, count) in exts.iter().take(15) {
                let bar_len = (count * 20 / exts[0].1).max(1) as usize;
                println!(
                    "  {:<12} {:>6}  {}",
                    ext.cyan(),
                    count.to_string().bold(),
                    "█".repeat(bar_len).green()
                );
            }
            println!("\n  Scanned in {}\n", elapsed(t).dimmed());
        }

        // ── organize ─────────────────────────────────────────────────────────
        Commands::Organize { folder, dry_run } => {
            let sp = spinner(if dry_run {
                "Previewing organisation…"
            } else {
                "Organising files…"
            });
            let ops = if dry_run {
                esprit_filesystem::organize_dry_run(&folder)?
            } else {
                esprit_filesystem::organize(&folder)?
            };
            sp.finish_and_clear();

            if ops.is_empty() {
                ok("Nothing to organise.");
            } else {
                let label = if dry_run { "Would move" } else { "Moved" };
                println!(
                    "\n  {} {} file{}\n",
                    "●".cyan(),
                    ops.len(),
                    if ops.len() == 1 { "" } else { "s" }
                );
                for op in &ops {
                    println!(
                        "  {} {} {} {}",
                        "→".dimmed(),
                        op.from
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .bold(),
                        "→".dimmed(),
                        op.to.display().to_string().cyan()
                    );
                }
                if dry_run {
                    println!(
                        "\n  {} Dry run — no files were moved. Remove --dry-run to apply.\n",
                        "⚠".yellow()
                    );
                } else {
                    println!("\n  {} {label} {} files.\n", "✓".green(), ops.len());
                }
            }
        }

        // ── duplicates ───────────────────────────────────────────────────────
        Commands::Duplicates { folder } => {
            let sp = spinner(&format!("Hashing files in {}…", folder.bold()));
            let t = Instant::now();
            let groups = esprit_filesystem::duplicates(&folder)?;
            sp.finish_and_clear();

            if groups.is_empty() {
                ok(&format!("No duplicates found.  {}", elapsed(t).dimmed()));
            } else {
                println!(
                    "\n  {} {} duplicate group{}  {}\n",
                    "⚠".yellow().bold(),
                    groups.len().to_string().bold(),
                    if groups.len() == 1 { "" } else { "s" },
                    elapsed(t).dimmed(),
                );
                for (i, group) in groups.iter().enumerate() {
                    println!("  Group {}", (i + 1).to_string().bold().yellow());
                    for file in group {
                        println!("    {} {}", "│".dimmed(), file.display().to_string().cyan());
                    }
                    println!();
                }
            }
        }

        // ── index ────────────────────────────────────────────────────────────
        Commands::Index { folder } => {
            let t = Instant::now();
            let sp = spinner(&format!("Indexing {}…", folder.bold()));
            let files = esprit_index::index(&folder);
            sp.finish_and_clear();

            match files {
                Err(e) => fail(&format!("Indexing failed: {e}")),
                Ok(files) => {
                    ok(&format!(
                        "Indexed {} files  {}",
                        files.len().to_string().bold(),
                        elapsed(t).dimmed()
                    ));
                    println!(
                        "  {} Run {} to build full-text search.\n",
                        "→".dimmed(),
                        "esprit rebuild".bold().cyan()
                    );
                }
            }
        }

        // ── rebuild ──────────────────────────────────────────────────────────
        Commands::Rebuild => {
            let t = Instant::now();
            let sp = spinner("Building full-text search index…");
            let res = esprit_index::rebuild_search_index();
            sp.finish_and_clear();

            match res {
                Err(e) => fail(&format!("Rebuild failed: {e}")),
                Ok(()) => ok(&format!("Search index built  {}", elapsed(t).dimmed())),
            }
        }

        // ── db ───────────────────────────────────────────────────────────────
        Commands::Db => {
            let files = esprit_index::all_files()?;
            println!(
                "\n  {} {} file{} in index\n",
                "●".cyan(),
                files.len().to_string().bold(),
                if files.len() == 1 { "" } else { "s" }
            );
            for file in &files {
                println!(
                    "  {}{:>10}  {}",
                    "│".dimmed(),
                    format_size(file.size),
                    file.path.display().to_string().cyan()
                );
            }
            println!();
        }

        // ── index-stats ──────────────────────────────────────────────────────
        Commands::IndexStats => {
            let s = esprit_index::index_stats()?;
            section("Index Summary");
            divider();
            kv("Files indexed", &s.file_count.to_string());
            kv(
                "Total size",
                &format!("{:.1} MB", s.total_bytes as f64 / 1_048_576.0),
            );
            println!();
        }

        // ── watch ────────────────────────────────────────────────────────────
        Commands::Watch { folder } => {
            println!(
                "\n  {} Watching {}  (press Ctrl-C to stop)\n",
                "👁".bold(),
                folder.bold().cyan()
            );
            esprit_platform::watch(&folder)?;
        }

        // ── ask ──────────────────────────────────────────────────────────────
        Commands::Ask { prompt, sources } => {
            // Optionally show sources first (pre-retrieval)
            if sources {
                let sp = spinner("Retrieving context…");
                let srcs = esprit_rag::source_files(&prompt).unwrap_or_default();
                sp.finish_and_clear();

                if !srcs.is_empty() {
                    println!("\n  {}", "Sources:".bold().dimmed());
                    for s in &srcs {
                        println!("  {} {}", "·".dimmed(), s.cyan());
                    }
                    println!();
                }
            }

            let sp = spinner("Thinking…");
            let t = Instant::now();
            let _ai = esprit_ai::Ai::default_model().expect("model load");
            sp.finish_and_clear();

            println!("\n  {}\n", "─".repeat(52).dimmed());
            print!("  "); // Initial indent

            let mut current_line_len = 2;
            let result = esprit_rag::ask_stream(&prompt, |chunk| {
                for c in chunk.chars() {
                    if c == '\n' {
                        println!();
                        print!("  ");
                        current_line_len = 2;
                    } else {
                        print!("{c}");
                        current_line_len += 1;
                        if current_line_len >= 74 && c.is_whitespace() {
                            println!();
                            print!("  ");
                            current_line_len = 2;
                        }
                    }
                }
                use std::io::Write;
                let _ = std::io::stdout().flush();
            });

            match result {
                Err(e) => {
                    fail(&format!("{e}"));
                    warn("Is Ollama running?  Try: esprit model pull <id>");
                }
                Ok((_, meta)) => {
                    println!("\n\n  {}\n", "─".repeat(52).dimmed());
                    println!(
                        "  {} {} tokens  {}",
                        "⏱".dimmed(),
                        meta.tokens.to_string().dimmed(),
                        elapsed(t).dimmed()
                    );
                    println!();
                }
            }
        }

        // ── agent ────────────────────────────────────────────────────────────
        Commands::Agent { agent, prompt } => {
            use esprit_agents::Agent;
            let ag = match agent.as_str() {
                "chat" => Agent::Chat,
                "code" => Agent::Code,
                "search" => Agent::Search,
                other => anyhow::bail!("unknown agent \"{other}\" — choose: chat | code | search"),
            };

            let sp = spinner(&format!("Running {} agent…", agent.bold()));
            let t = Instant::now();
            let res = esprit_agents::run(ag, &prompt);
            sp.finish_and_clear();

            match res {
                Err(e) => fail(&e.to_string()),
                Ok(out) => {
                    println!("\n  {}\n", "─".repeat(52).dimmed());
                    for line in out.lines() {
                        println!("  {line}");
                    }
                    println!("\n  {}  {}\n", "─".repeat(52).dimmed(), elapsed(t).dimmed());
                }
            }
        }

        // ── workflow ─────────────────────────────────────────────────────────
        Commands::Workflow { workflow, prompt } => {
            let sp = spinner(&format!("Running {} workflow…", workflow.bold()));
            let t = Instant::now();
            let out = match workflow.as_str() {
                "explain" => esprit_workflows::explain(&prompt),
                "review" => esprit_workflows::code_review(&prompt),
                "search" => esprit_workflows::project_search(&prompt),
                other => anyhow::bail!(
                    "unknown workflow \"{other}\" — choose: explain | review | search"
                ),
            };
            sp.finish_and_clear();

            match out {
                Err(e) => fail(&e.to_string()),
                Ok(text) => {
                    println!("\n  {}\n", "─".repeat(52).dimmed());
                    for line in text.lines() {
                        println!("  {line}");
                    }
                    println!("\n  {}  {}\n", "─".repeat(52).dimmed(), elapsed(t).dimmed());
                }
            }
        }

        // ── memory-clear ─────────────────────────────────────────────────────
        Commands::MemoryClear => {
            let n = esprit_memory::clear()?;
            ok(&format!("Cleared {n} memory entries."));
        }

        // ── memory-stats ─────────────────────────────────────────────────────
        Commands::MemoryStats => {
            let n = esprit_memory::count()?;
            section("Conversation Memory");
            divider();
            kv("Stored exchanges", &n.to_string());
            println!();
        }

        // ── diary ────────────────────────────────────────────────────────────
        Commands::Diary { note } => {
            let branch = esprit_platform::doctor::capture("git", &["branch", "--show-current"])
                .unwrap_or_else(|| "main".to_string());
            let branch = branch.trim();

            if let Some(content) = note {
                diary::add_note(branch, &content)?;
                ok(&format!("Saved note to branch '{}'", branch.bold()));
            } else {
                let notes = diary::list_notes(branch)?;
                section(&format!("Diary notes for '{}'", branch.bold()));
                divider();
                if notes.is_empty() {
                    println!("  (No notes found)");
                } else {
                    for (i, n) in notes.iter().enumerate() {
                        println!("  {}. {}", i + 1, n);
                    }
                }
                println!();
            }
        }

        // ── dashboard ────────────────────────────────────────────────────────
        Commands::Graph => {
            let graph = esprit_index::graph::build_graph()?;
            let mermaid = esprit_index::graph::to_mermaid(&graph);
            println!("{mermaid}");
        }
        Commands::Deps => {
            let graph = esprit_index::graph::build_graph()?;
            section("Project Dependencies (from index)");
            divider();
            for edge in graph.graph.edge_indices() {
                if let Some((s_idx, t_idx)) = graph.graph.edge_endpoints(edge) {
                    let src = &graph.graph[s_idx];
                    let tgt = &graph.graph[t_idx];
                    let kind = &graph.graph[edge];
                    println!("  {src} -> {tgt} ({kind})");
                }
            }
            println!();
        }
        Commands::TestGaps => {
            section("Test Gap Finder");
            divider();
            let files = esprit_index::all_files()?;
            let mut sources = Vec::new();
            let mut tests = Vec::new();
            for f in files {
                let s = f.path.to_string_lossy().to_string();
                if s.contains("test") || s.contains("spec") {
                    tests.push(s);
                } else if s.ends_with(".rs") || s.ends_with(".js") || s.ends_with(".ts") || s.ends_with(".py") {
                    sources.push(s);
                }
            }
            let mut gaps = 0;
            for src in &sources {
                let name = std::path::Path::new(src).file_stem().unwrap_or_default().to_string_lossy();
                let has_test = tests.iter().any(|t| t.contains(&*name));
                if !has_test {
                    println!("  {} Missing test for: {}", "○".dimmed(), src);
                    gaps += 1;
                }
            }
            println!("\n  Found {gaps} source files without matching test files.\n");
        }

        Commands::Dashboard => {
            dashboard::run()?;
        }

        // ── init ─────────────────────────────────────────────────────────────
        Commands::Init { with_embeddings } => {
            println!("{}", esprit_core::banner().cyan().bold());
            println!("\n  Setting up Esprit — this downloads AI models on first run.\n");

            let llm = esprit_models::default_llm();
            if esprit_models::is_installed(llm)? {
                ok(&format!("{} is already installed.", llm.display));
            } else {
                println!(
                    "  {} Downloading {} (~{} MB)…\n",
                    "⬇".cyan().bold(),
                    llm.display.bold(),
                    llm.size_bytes / 1_000_000
                );
                esprit_models::pull(llm)?;
            }

            if with_embeddings {
                let emb = esprit_models::default_embed();
                if esprit_models::is_installed(emb)? {
                    ok(&format!("{} is already installed.", emb.display));
                } else {
                    println!(
                        "\n  {} Downloading {} (~{} MB)…\n",
                        "⬇".cyan().bold(),
                        emb.display.bold(),
                        emb.size_bytes / 1_000_000
                    );
                    esprit_models::pull(emb)?;
                }
            }

            println!();
            ok("Esprit is ready.");
            println!(
                "\n  {} Try: {}\n  {} Try: {}\n",
                "→".dimmed(),
                "esprit ask \"what does this project do?\"".bold().cyan(),
                "→".dimmed(),
                "esprit doctor".bold().cyan(),
            );
        }

        // ── model ─────────────────────────────────────────────────────────────
        Commands::Model { action } => match action {
            ModelAction::List => {
                section("Available Models");
                divider();
                for (entry, installed) in esprit_models::list_status()? {
                    let status = if installed {
                        "✓ installed".green().bold().to_string()
                    } else {
                        "○ not installed".dimmed().to_string()
                    };
                    let kind = match entry.kind {
                        esprit_models::ModelKind::Llm => "LLM",
                        esprit_models::ModelKind::Embedding => "Embed",
                    };
                    println!(
                        "  {:<14} {:<8} {:<30} {}",
                        entry.id.bold(),
                        kind.dimmed(),
                        entry.display,
                        status
                    );
                }
                println!(
                    "\n  Pull a model: {}\n",
                    "esprit model pull <id>".bold().cyan()
                );
            }

            ModelAction::Pull { id } => {
                let entry = esprit_models::lookup(&id).ok_or_else(|| {
                    anyhow::anyhow!("Unknown model \"{id}\" — run `esprit model list`")
                })?;
                println!(
                    "\n  {} Downloading {}…\n",
                    "⬇".cyan().bold(),
                    entry.display.bold()
                );
                esprit_models::pull(entry)?;
                ok("Done.");
            }

            ModelAction::Remove { id } => {
                let entry = esprit_models::lookup(&id)
                    .ok_or_else(|| anyhow::anyhow!("Unknown model \"{id}\""))?;
                esprit_models::remove(entry)?;
                ok(&format!("Removed {}.", entry.display));
            }
        },
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1}M", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.0}K", bytes as f64 / 1_024.0)
    } else {
        format!("{bytes}B")
    }
}

// ── Developer Diary ────────────────────────────────────────────────────────
pub mod diary {
    use anyhow::Result;
    use rusqlite::{params, Connection};
    use std::fs;

    fn db() -> Result<Connection> {
        let dir = esprit_config::Config::load()?.workspace.join(".esprit");
        fs::create_dir_all(&dir)?;
        let conn = Connection::open(dir.join("diary.db"))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS diary (
                id INTEGER PRIMARY KEY,
                branch TEXT NOT NULL,
                note TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (unixepoch())
            )",
            [],
        )?;
        Ok(conn)
    }

    pub fn add_note(branch: &str, note: &str) -> Result<()> {
        let conn = db()?;
        conn.execute(
            "INSERT INTO diary (branch, note) VALUES (?1, ?2)",
            params![branch, note],
        )?;
        Ok(())
    }

    pub fn list_notes(branch: &str) -> Result<Vec<String>> {
        let conn = db()?;
        let mut stmt = conn.prepare("SELECT note FROM diary WHERE branch = ?1 ORDER BY created_at DESC")?;
        let rows = stmt.query_map([branch], |r| r.get(0))?;
        Ok(rows.filter_map(Result::ok).collect())
    }
}

// ── Dashboard ────────────────────────────────────────────────────────
pub mod dashboard {
    use anyhow::Result;
    use crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
        Terminal,
    };
    use std::io;

    pub fn run() -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.area());

                let stats = format!(
                    "Models Installed: {}\nVectors Indexed: {}\nConversations in Memory: {}\n",
                    esprit_models::list_status().unwrap_or_default().iter().filter(|(_, x)| *x).count(),
                    esprit_vectors::count().unwrap_or(0),
                    esprit_memory::count().unwrap_or(0)
                );

                let top = Paragraph::new(stats)
                    .block(Block::default().title("Esprit System Health").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Cyan));
                
                let bottom = Paragraph::new("Press 'q' to quit.")
                    .block(Block::default().title("Controls").borders(Borders::ALL));

                f.render_widget(top, chunks[0]);
                f.render_widget(bottom, chunks[1]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
}
