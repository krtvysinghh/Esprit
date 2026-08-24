use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow!("unable to determine data directory"))
}

pub(crate) fn database_path() -> Result<PathBuf> {
    let d = dirs()?;
    fs::create_dir_all(d.data_dir())?;
    Ok(d.data_dir().join("index.db"))
}

pub(crate) fn open_database() -> Result<Connection> {
    let conn = Connection::open(database_path()?)?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         CREATE TABLE IF NOT EXISTS files(
             path         TEXT    PRIMARY KEY,
             size         INTEGER NOT NULL,
             mtime        INTEGER NOT NULL DEFAULT 0,
             content_hash TEXT,
             language     TEXT
         );",
    )?;
    Ok(conn)
}

/// Detect a file's language from its extension.
pub(crate) fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "Rust",
        "py" => "Python",
        "js" | "mjs" | "cjs" => "JavaScript",
        "ts" | "tsx" => "TypeScript",
        "go" => "Go",
        "java" => "Java",
        "kt" => "Kotlin",
        "c" | "h" => "C",
        "cpp" | "cc" | "cxx" | "hpp" => "C++",
        "cs" => "C#",
        "rb" => "Ruby",
        "php" => "PHP",
        "swift" => "Swift",
        "scala" => "Scala",
        "toml" => "TOML",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "md" | "mdx" => "Markdown",
        "html" | "htm" => "HTML",
        "css" | "scss" | "sass" => "CSS",
        "sh" | "bash" | "zsh" => "Shell",
        "sql" => "SQL",
        _ => "Unknown",
    }
}

pub fn insert_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if !path.is_file() {
        return Ok(());
    }
    let meta = path.metadata()?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let lang = detect_language(path);
    let conn = open_database()?;
    conn.execute(
        "INSERT OR REPLACE INTO files(path,size,mtime,language) VALUES(?1,?2,?3,?4)",
        params![path.to_string_lossy(), meta.len(), mtime, lang],
    )?;
    Ok(())
}

pub fn update_file(path: impl AsRef<Path>) -> Result<()> {
    insert_file(path)
}

pub fn delete_file(path: impl AsRef<Path>) -> Result<()> {
    let conn = open_database()?;
    conn.execute(
        "DELETE FROM files WHERE path=?1",
        params![path.as_ref().to_string_lossy()],
    )?;
    Ok(())
}

pub fn rename_file(old: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {
    let new = new.as_ref();
    let meta = new.metadata()?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let conn = open_database()?;
    conn.execute(
        "UPDATE files SET path=?1,size=?2,mtime=?3 WHERE path=?4",
        params![
            new.to_string_lossy(),
            meta.len(),
            mtime,
            old.as_ref().to_string_lossy()
        ],
    )?;
    Ok(())
}
