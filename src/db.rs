use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding BLOB,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;
    Ok(conn)
}
