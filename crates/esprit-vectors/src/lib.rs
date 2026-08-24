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
    let c = Connection::open(d.data_dir().join("vectors.db"))?;
    c.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         CREATE TABLE IF NOT EXISTS vectors(
             id        INTEGER PRIMARY KEY,
             key       TEXT    UNIQUE NOT NULL,
             embedding TEXT    NOT NULL
         );",
    )?;
    Ok(c)
}

/// Store a named embedding vector.
pub fn store(key: &str, v: &[f32]) -> Result<()> {
    db()?.execute(
        "INSERT OR REPLACE INTO vectors(key,embedding) VALUES(?1,?2)",
        params![key, serde_json::to_string(v)?],
    )?;
    Ok(())
}

/// Load a named embedding vector by exact key.
pub fn load(key: &str) -> Result<Option<Vec<f32>>> {
    let c = db()?;
    let mut s = c.prepare("SELECT embedding FROM vectors WHERE key=?1")?;
    let mut r = s.query([key])?;
    if let Some(row) = r.next()? {
        let t: String = row.get(0)?;
        return Ok(Some(serde_json::from_str(&t)?));
    }
    Ok(None)
}

/// Cosine similarity between two vectors (returns 0.0 if either is empty).
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}

/// Brute-force nearest-neighbour search.
/// Returns the top-`k` (key, score) pairs sorted by descending cosine similarity.
pub fn nearest(query: &[f32], k: usize) -> Result<Vec<(String, f32)>> {
    let c = db()?;
    let mut stmt = c.prepare("SELECT key, embedding FROM vectors")?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;

    let mut scores: Vec<(String, f32)> = rows
        .filter_map(|r| r.ok())
        .filter_map(|(key, raw)| {
            let v: Vec<f32> = serde_json::from_str(&raw).ok()?;
            Some((key, cosine(query, &v)))
        })
        .collect();

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(k);
    Ok(scores)
}

/// Total number of stored vectors.
pub fn count() -> Result<i64> {
    let conn = db()?;
    Ok(conn.query_row("SELECT COUNT(*) FROM vectors", [], |r| r.get(0))?)
}
