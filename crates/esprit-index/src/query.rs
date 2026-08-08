use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use std::path::PathBuf;

pub fn all_files() -> Result<Vec<IndexedFile>> {
    let conn = open_database()?;

    let mut stmt = conn.prepare("SELECT path,size FROM files ORDER BY path")?;

    let rows = stmt.query_map([], |row| {
        Ok(IndexedFile {
            path: PathBuf::from(row.get::<_, String>(0)?),
            size: row.get(1)?,
        })
    })?;

    let mut files = Vec::new();

    for row in rows {
        files.push(row?);
    }

    Ok(files)
}
