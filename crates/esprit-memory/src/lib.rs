use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;

fn dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow!("unable to determine data directory"))
}

fn db() -> Result<Connection> {
    let d = dirs()?;
    fs::create_dir_all(d.data_dir())?;
    let conn = Connection::open(d.data_dir().join("memory.db"))?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         CREATE TABLE IF NOT EXISTS memory(
             id        INTEGER PRIMARY KEY,
             session   TEXT    NOT NULL DEFAULT 'default',
             question  TEXT    NOT NULL,
             answer    TEXT    NOT NULL,
             created_at INTEGER NOT NULL DEFAULT(unixepoch())
         );",
    )?;
    Ok(conn)
}

/// Persist a question/answer pair for the given session.
pub fn remember(question: &str, answer: &str) -> Result<()> {
    remember_session("default", question, answer)
}

/// Persist a question/answer pair tagged with a session name.
pub fn remember_session(session: &str, question: &str, answer: &str) -> Result<()> {
    db()?.execute(
        "INSERT INTO memory(session,question,answer) VALUES(?1,?2,?3)",
        params![session, question, answer],
    )?;
    Ok(())
}

/// Recall the most recent `limit` exchanges (newest first) for the default session.
pub fn recall(limit: usize) -> Result<Vec<(String, String)>> {
    recall_session("default", limit)
}

/// Recall the most recent `limit` exchanges for a named session.
pub fn recall_session(session: &str, limit: usize) -> Result<Vec<(String, String)>> {
    let conn = db()?;
    let mut stmt = conn.prepare(
        "SELECT question, answer
         FROM memory
         WHERE session = ?1
         ORDER BY id DESC
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![session, limit as i64], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

/// Erase all memory for the default session.
pub fn clear() -> Result<usize> {
    clear_session("default")
}

/// Erase all memory for a named session.
pub fn clear_session(session: &str) -> Result<usize> {
    Ok(db()?.execute("DELETE FROM memory WHERE session=?1", [session])?)
}

/// Total number of stored exchanges.
pub fn count() -> Result<i64> {
    let conn = db()?;
    Ok(conn.query_row("SELECT COUNT(*) FROM memory", [], |r| r.get(0))?)
}
