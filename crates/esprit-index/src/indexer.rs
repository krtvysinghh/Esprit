use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use ignore::WalkBuilder;
use rusqlite::params;
use std::path::Path;

/// Index all files under `root`, respecting `.gitignore`, `.ignore`, and
/// other standard ignore rules. Uses incremental update: files whose
/// modification time has not changed since the last index are skipped.
pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let root = root.as_ref();
    let conn = open_database()?;

    let mut files = Vec::new();

    let walker = WalkBuilder::new(root)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(false) // include dotfiles (but not .git contents)
        .build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("walk error: {e}");
                continue;
            }
        };

        if entry.file_type().map(|ft| !ft.is_file()).unwrap_or(true) {
            continue;
        }

        let path = entry.path();

        // Skip .git directory contents explicitly
        if path
            .components()
            .any(|c| c.as_os_str() == ".git" || c.as_os_str() == "target")
        {
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("metadata error for {}: {e}", path.display());
                continue;
            }
        };

        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let path_str = path.to_string_lossy().to_string();
        let lang = crate::database::detect_language(path);

        // Incremental: skip if mtime is unchanged
        let existing_mtime: Option<i64> = conn
            .query_row("SELECT mtime FROM files WHERE path=?1", [&path_str], |r| {
                r.get(0)
            })
            .ok();

        if existing_mtime == Some(mtime) {
            files.push(IndexedFile {
                path: path.to_path_buf(),
                size: meta.len(),
            });
            continue;
        }

        conn.execute(
            "INSERT OR REPLACE INTO files(path,size,mtime,language) VALUES(?1,?2,?3,?4)",
            params![path_str, meta.len(), mtime, lang],
        )?;

        files.push(IndexedFile {
            path: path.to_path_buf(),
            size: meta.len(),
        });
    }

    Ok(files)
}
