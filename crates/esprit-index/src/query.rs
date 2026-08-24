use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use std::path::PathBuf;

/// Return all indexed files ordered by path.
pub fn all_files() -> Result<Vec<IndexedFile>> {
    let conn = open_database()?;
    let mut stmt = conn.prepare("SELECT path,size FROM files ORDER BY path")?;
    let rows = stmt.query_map([], |row| {
        Ok(IndexedFile {
            path: PathBuf::from(row.get::<_, String>(0)?),
            size: row.get(1)?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

/// Return aggregate statistics about the index.
pub fn index_stats() -> Result<IndexStats> {
    let conn = open_database()?;
    let file_count: i64 = conn.query_row("SELECT COUNT(*) FROM files", [], |r| r.get(0))?;
    let total_bytes: i64 =
        conn.query_row("SELECT COALESCE(SUM(size),0) FROM files", [], |r| r.get(0))?;
    Ok(IndexStats {
        file_count,
        total_bytes,
    })
}

#[derive(Debug)]
pub struct IndexStats {
    pub file_count: i64,
    pub total_bytes: i64,
}
