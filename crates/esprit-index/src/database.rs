use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn database_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;

    fs::create_dir_all(dirs.data_dir())?;
    Ok(dirs.data_dir().join("index.db"))
}

pub(crate) fn open_database() -> Result<Connection> {
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

pub fn insert_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !path.is_file() {
        return Ok(());
    }

    let conn = open_database()?;

    conn.execute(
        "INSERT OR REPLACE INTO files(path,size) VALUES(?1,?2)",
        params![path.to_string_lossy(), path.metadata()?.len()],
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
    let conn = open_database()?;

    conn.execute(
        "UPDATE files SET path=?1,size=?2 WHERE path=?3",
        params![
            new.as_ref().to_string_lossy(),
            new.as_ref().metadata()?.len(),
            old.as_ref().to_string_lossy()
        ],
    )?;

    Ok(())
}
