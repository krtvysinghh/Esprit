use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

pub fn open(name: &str) -> Result<Connection> {
    let d = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine storage directory"))?;

    fs::create_dir_all(d.data_dir())?;

    let conn = Connection::open(d.data_dir().join(name))?;

    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "FULL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    Ok(conn)
}
