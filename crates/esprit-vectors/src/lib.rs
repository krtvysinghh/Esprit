use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;

fn db() -> Result<Connection> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit").unwrap();
    fs::create_dir_all(dirs.data_dir())?;
    let c = Connection::open(dirs.data_dir().join("vectors.db"))?;
    c.execute_batch(
        r#"
 CREATE TABLE IF NOT EXISTS vectors(
 id INTEGER PRIMARY KEY,
 key TEXT UNIQUE,
 embedding TEXT
 );
 "#,
    )?;
    Ok(c)
}

pub fn store(key: &str, v: &[f32]) -> Result<()> {
    db()?.execute(
        "INSERT OR REPLACE INTO vectors(key,embedding) VALUES(?1,?2)",
        params![key, serde_json::to_string(v)?],
    )?;
    Ok(())
}

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
