use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

/// Returns the canonical project directories, propagating errors instead of panicking.
pub fn dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow!("unable to determine data directory"))
}

/// Opens (or creates) a named SQLite database in the Esprit data directory.
/// WAL mode is enabled automatically for all connections.
pub fn open(name: &str) -> Result<Connection> {
    let d = dirs()?;
    let data = d.data_dir();
    fs::create_dir_all(data)?;
    let conn = Connection::open(data.join(name))?;
    // WAL mode: concurrent watcher + CLI access without SQLITE_BUSY
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
    Ok(conn)
}
