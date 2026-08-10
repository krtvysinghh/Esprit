use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use rusqlite::params;
use std::path::Path;
use walkdir::WalkDir;

pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let mut conn = open_database()?;

    let transaction = conn.transaction()?;
    transaction.execute("DELETE FROM files", [])?;
    let mut insert = transaction.prepare("INSERT INTO files(path,size) VALUES(?1,?2)")?;

    let mut files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_entry(|entry| {
        let name = entry.file_name().to_string_lossy();
        name != ".git" && name != "target"
    }) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        if entry.file_name() == "index.db" {
            continue;
        }

        let file = IndexedFile {
            path: entry.path().to_path_buf(),
            size: entry.metadata()?.len(),
        };

        insert.execute(params![file.path.to_string_lossy(), file.size])?;

        files.push(file);
    }

    drop(insert);
    transaction.commit()?;

    Ok(files)
}
