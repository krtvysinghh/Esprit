use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use std::path::PathBuf;

pub fn all_files() -> Result<Vec<IndexedFile>> {
    let conn = open_database()?;

    let mut stmt = conn.prepare("SELECT path,size,modified FROM files ORDER BY path")?;

    let rows = stmt.query_map([], |row| {
        Ok(IndexedFile {
            path: PathBuf::from(row.get::<_, String>(0)?),
            size: row.get(1)?,
            modified: row.get(2)?,
        })
    })?;

    let mut files = Vec::new();

    for row in rows {
        files.push(row?);
    }

    Ok(files)
}

pub fn file_relations(source: impl AsRef<std::path::Path>) -> Result<Vec<crate::FileRelation>> {
    let conn = crate::database::open_database()?;

    let mut stmt = conn.prepare(
        "SELECT source,target,relation
         FROM file_relations
         WHERE source=?1
         ORDER BY target,relation",
    )?;

    let rows = stmt.query_map(
        rusqlite::params![source.as_ref().to_string_lossy().to_string()],
        |row| {
            Ok(crate::FileRelation {
                source: std::path::PathBuf::from(row.get::<_, String>(0)?),
                target: std::path::PathBuf::from(row.get::<_, String>(1)?),
                relation: row.get(2)?,
            })
        },
    )?;

    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn index_health() -> Result<crate::IndexHealth> {
    let conn = crate::database::open_database()?;

    let indexed_files: usize =
        conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get::<_, i64>(0))? as usize;

    let mut stmt = conn.prepare("SELECT path,modified FROM files")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            std::path::PathBuf::from(row.get::<_, String>(0)?),
            row.get::<_, i64>(1)?,
        ))
    })?;

    let mut missing_files = 0usize;
    let mut stale_files = 0usize;

    for row in rows {
        let (path, stored_modified) = row?;

        match std::fs::metadata(&path) {
            Ok(metadata) => {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        if duration.as_secs() as i64 != stored_modified {
                            stale_files += 1;
                        }
                    }
                }
            }
            Err(_) => {
                missing_files += 1;
            }
        }
    }

    Ok(crate::IndexHealth {
        indexed_files,
        missing_files,
        stale_files,
        healthy: missing_files == 0 && stale_files == 0,
    })
}

pub fn files_in_workspace(root: impl AsRef<std::path::Path>) -> Result<Vec<crate::IndexedFile>> {
    let root = root.as_ref().canonicalize()?;
    let root_text = root.to_string_lossy().to_string();

    let conn = crate::database::open_database()?;
    let pattern = format!("{}/%", root_text.trim_end_matches('/'));

    let mut stmt = conn.prepare(
        "SELECT path,size,modified
         FROM files
         WHERE path=?1 OR path LIKE ?2
         ORDER BY path",
    )?;

    let rows = stmt.query_map(rusqlite::params![root_text, pattern], |row| {
        Ok(crate::IndexedFile {
            path: std::path::PathBuf::from(row.get::<_, String>(0)?),
            size: row.get::<_, u64>(1)?,
            modified: row.get::<_, u64>(2)?,
        })
    })?;

    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn recover_index(root: impl AsRef<std::path::Path>) -> Result<Vec<crate::IndexedFile>> {
    let root = root.as_ref();

    crate::verify_database_integrity()?;

    let indexed = crate::indexer::index(root)?;

    crate::rebuild_search_index()?;

    Ok(indexed)
}
