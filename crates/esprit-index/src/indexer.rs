use crate::{IndexedFile, database::open_database};
use anyhow::Result;
use rusqlite::params;
use std::path::Path;
use walkdir::WalkDir;

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
            params![file.path.to_string_lossy(), file.size],
        )?;

        files.push(file);
    }

    Ok(files)
}
