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

        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS files(
                path TEXT PRIMARY KEY,
                size INTEGER NOT NULL,
                modified INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS file_relations(
                source TEXT NOT NULL,
                target TEXT NOT NULL,
                relation TEXT NOT NULL,
                PRIMARY KEY(source,target,relation)
            );

            CREATE INDEX IF NOT EXISTS idx_file_relations_source
                ON file_relations(source);

            CREATE INDEX IF NOT EXISTS idx_file_relations_target
                ON file_relations(target);


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

        let has_modified: bool = conn
            .prepare("PRAGMA table_info(files)")?
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<rusqlite::Result<Vec<_>>>()?
            .iter()
            .any(|name| name == "modified");

        if !has_modified {
            conn.execute(
                "ALTER TABLE files ADD COLUMN modified INTEGER NOT NULL DEFAULT 0",
                [],
            )?;
        }

        Ok(Self { conn })
    }

    pub fn transaction<F>(&mut self, operation: F) -> Result<()>
    where
        F: FnOnce(&rusqlite::Transaction<'_>) -> Result<()>,
    {
        let tx = self.conn.transaction()?;
        operation(&tx)?;
        tx.commit()?;
        Ok(())
    }

    pub fn insert_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if !path.is_file() {
            return Ok(());
        }

        let mut statement = self
            .conn
            .prepare_cached("INSERT OR REPLACE INTO files(path,size,modified) VALUES(?1,?2,?3)")?;

        let metadata = path.metadata()?;
        let modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        statement.execute(params![path.to_string_lossy(), metadata.len(), modified])?;

        Ok(())
    }

    pub fn update_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if !path.is_file() {
            return Ok(());
        }

        let mut statement = self
            .conn
            .prepare_cached("UPDATE files SET size=?1,modified=?2 WHERE path=?3")?;

        let metadata = path.metadata()?;
        let modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        statement.execute(params![metadata.len(), modified, path.to_string_lossy()])?;

        Ok(())
    }

    pub fn add_relation(
        &self,
        source: impl AsRef<Path>,
        target: impl AsRef<Path>,
        relation: impl AsRef<str>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO file_relations(source,target,relation)
             VALUES(?1,?2,?3)",
            params![
                source.as_ref().to_string_lossy().to_string(),
                target.as_ref().to_string_lossy().to_string(),
                relation.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_relations(&self, source: impl AsRef<Path>) -> Result<()> {
        self.conn.execute(
            "DELETE FROM file_relations WHERE source=?1",
            params![source.as_ref().to_string_lossy().to_string()],
        )?;
        Ok(())
    }

    pub fn verify_integrity(&self) -> Result<()> {
        self.conn.execute_batch(
            "PRAGMA foreign_keys=ON;
             PRAGMA integrity_check;",
        )?;

        let result: String = self
            .conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))?;

        if result != "ok" {
            return Err(anyhow::anyhow!("SQLite integrity check failed: {}", result));
        }

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
