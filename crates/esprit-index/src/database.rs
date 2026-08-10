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

pub struct IndexDatabase {
    conn: Connection,
}

impl IndexDatabase {
    pub fn open() -> Result<Self> {
        let conn = Connection::open(database_path()?)?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS files(
                path TEXT PRIMARY KEY,
                size INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS file_links(
                source TEXT NOT NULL,
                target TEXT NOT NULL,
                kind TEXT NOT NULL,
                PRIMARY KEY(source, target, kind),
                FOREIGN KEY(source) REFERENCES files(path) ON DELETE CASCADE,
                FOREIGN KEY(target) REFERENCES files(path) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_file_links_source
                ON file_links(source);

            CREATE INDEX IF NOT EXISTS idx_file_links_target
                ON file_links(target);
            ",
        )?;

        Ok(Self { conn })
    }

    pub fn insert_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if !path.is_file() {
            return Ok(());
        }

        let mut statement = self
            .conn
            .prepare_cached("INSERT OR REPLACE INTO files(path,size) VALUES(?1,?2)")?;

        statement.execute(params![path.to_string_lossy(), path.metadata()?.len()])?;

        Ok(())
    }

    pub fn update_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if !path.is_file() {
            return Ok(());
        }

        let mut statement = self
            .conn
            .prepare_cached("UPDATE files SET size=?1 WHERE path=?2")?;

        statement.execute(params![path.metadata()?.len(), path.to_string_lossy()])?;

        Ok(())
    }

    pub fn delete_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut statement = self
            .conn
            .prepare_cached("DELETE FROM files WHERE path=?1")?;

        statement.execute(params![path.as_ref().to_string_lossy()])?;

        Ok(())
    }

    pub fn rename_file(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {
        let mut statement = self
            .conn
            .prepare_cached("UPDATE files SET path=?1,size=?2 WHERE path=?3")?;

        statement.execute(params![
            new.as_ref().to_string_lossy(),
            new.as_ref().metadata()?.len(),
            old.as_ref().to_string_lossy()
        ])?;

        Ok(())
    }
}

pub(crate) fn open_database() -> Result<Connection> {
    Ok(IndexDatabase::open()?.conn)
}

pub fn insert_file(path: impl AsRef<Path>) -> Result<()> {
    IndexDatabase::open()?.insert_file(path)
}

pub fn update_file(path: impl AsRef<Path>) -> Result<()> {
    IndexDatabase::open()?.update_file(path)
}

pub fn delete_file(path: impl AsRef<Path>) -> Result<()> {
    IndexDatabase::open()?.delete_file(path)
}

pub fn rename_file(old: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {
    IndexDatabase::open()?.rename_file(old, new)
}
