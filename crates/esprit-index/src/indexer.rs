use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use rusqlite::params;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let root = root.as_ref();
    let mut conn = open_database()?;

    let mut existing = HashMap::<PathBuf, (u64, u64)>::new();
    {
        let mut stmt = conn.prepare("SELECT path,size,modified FROM files")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                PathBuf::from(row.get::<_, String>(0)?),
                row.get::<_, u64>(1)?,
                row.get::<_, u64>(2)?,
            ))
        })?;

        for row in rows {
            let (path, size, modified) = row?;
            existing.insert(path, (size, modified));
        }
    }

    let transaction = conn.transaction()?;
    let mut upsert =
        transaction.prepare("INSERT OR REPLACE INTO files(path,size,modified) VALUES(?1,?2,?3)")?;
    let mut remove = transaction.prepare("DELETE FROM files WHERE path=?1")?;

    let mut files = Vec::new();
    let mut seen = HashSet::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != ".git" && name != "target"
        })
    {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        if entry.file_name() == "index.db" {
            continue;
        }

        let metadata = entry.metadata()?;
        let modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let file = IndexedFile {
            path: entry.path().to_path_buf(),
            size: metadata.len(),
            modified,
        };

        seen.insert(file.path.clone());

        let unchanged = existing
            .get(&file.path)
            .map(|(size, previous_modified)| {
                *size == file.size && *previous_modified == file.modified
            })
            .unwrap_or(false);

        if !unchanged {
            upsert.execute(params![
                file.path.to_string_lossy(),
                file.size,
                file.modified
            ])?;
        }

        files.push(file);
    }

    for path in existing.keys() {
        if !seen.contains(path) {
            remove.execute(params![path.to_string_lossy()])?;
        }
    }

    drop(remove);
    drop(upsert);
    transaction.commit()?;

    Ok(files)
}
