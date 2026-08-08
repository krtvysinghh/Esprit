use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;

fn db() -> Result<Connection> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit").unwrap();
    fs::create_dir_all(dirs.data_dir())?;
    let conn = Connection::open(dirs.data_dir().join("memory.db"))?;

    conn.execute_batch(
        r#"
    CREATE TABLE IF NOT EXISTS memory(
        id INTEGER PRIMARY KEY,
        question TEXT,
        answer TEXT,
        created_at INTEGER DEFAULT(unixepoch())
    );
    "#,
    )?;

    Ok(conn)
}

pub fn remember(question: &str, answer: &str) -> Result<()> {
    db()?
        .execute("INSERT INTO memory(question,answer) VALUES(?1,?2)", params![question, answer])?;
    Ok(())
}

pub fn recall(limit: usize) -> Result<Vec<(String, String)>> {
    let conn = db()?;

    let mut stmt = conn.prepare(
        "SELECT question,answer
         FROM memory
         ORDER BY id DESC
         LIMIT ?1",
    )?;

    let rows = stmt.query_map([limit as i64], |r| Ok((r.get(0)?, r.get(1)?)))?;

    Ok(rows.filter_map(Result::ok).collect())
}
