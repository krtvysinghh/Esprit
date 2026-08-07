use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub path: PathBuf,
    pub size: u64,
}

fn database_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;

    fs::create_dir_all(dirs.data_dir())?;

    Ok(dirs.data_dir().join("index.db"))
}

fn open_database() -> Result<Connection> {
    let conn = Connection::open(database_path()?)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS files(
            path TEXT PRIMARY KEY,
            size INTEGER NOT NULL
        );
        ",
    )?;

    Ok(conn)
}

pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let conn = open_database()?;

    conn.execute("DELETE FROM files", [])?;

    let mut files = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let file = IndexedFile { path: entry.path().to_path_buf(), size: entry.metadata()?.len() };

        conn.execute(
            "INSERT INTO files(path,size) VALUES(?1,?2)",
            (&file.path.to_string_lossy(), file.size),
        )?;

        files.push(file);
    }

    Ok(files)
}

pub fn all_files() -> Result<Vec<IndexedFile>> {
    let conn = open_database()?;

    let mut stmt = conn.prepare("SELECT path,size FROM files ORDER BY path")?;

    let rows = stmt.query_map([], |row| {
        Ok(IndexedFile { path: PathBuf::from(row.get::<_, String>(0)?), size: row.get(1)? })
    })?;

    let mut files = Vec::new();

    for row in rows {
        files.push(row?);
    }

    Ok(files)
}
