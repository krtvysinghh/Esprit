use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use esprit_platform::doctor::capture;

pub fn run_daemon() -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    
    // Watch current workspace
    let path = std::env::current_dir()?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    println!("⚡ Esprit System Daemon started.");
    println!("Watching workspace: {}", path.display());
    
    let mut last_git_status = String::new();
    let mut last_reindex = Instant::now() - Duration::from_secs(60); // Allow immediate first reindex

    loop {
        // Handle file system events
        if let Ok(Ok(event)) = rx.recv_timeout(Duration::from_secs(5)) {
            match event.kind {
                notify::EventKind::Modify(_) | notify::EventKind::Create(_) | notify::EventKind::Remove(_) => {
                    let mut needs_reindex = false;
                    for path in &event.paths {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        if ["rs", "js", "ts", "py", "md", "toml", "json"].contains(&ext) {
                            needs_reindex = true;
                            break;
                        }
                    }
                    if needs_reindex && last_reindex.elapsed() > Duration::from_secs(30) {
                        println!("[Daemon] File change detected. Triggering debounced re-indexing...");
                        let _ = esprit_index::rebuild_search_index();
                        last_reindex = Instant::now();
                    }
                }
                _ => {}
            }
        }

        // Handle Background Git Watcher
        if let Some(status) = capture("git", &["status", "--short"]) {
            if status != last_git_status {
                if !status.is_empty() {
                    println!("[Daemon Git Watcher] Worktree modified. Pair programmer context updated.");
                }
                last_git_status = status;
            }
        }
    }
}
