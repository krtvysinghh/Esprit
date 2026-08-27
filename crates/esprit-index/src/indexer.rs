use crate::{database::open_database, IndexedFile};
use anyhow::Result;
use ignore::WalkBuilder;
use rusqlite::params;
use std::path::Path;

pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let root = root.as_ref();
    let conn = open_database()?;

    let mut files = Vec::new();

    let walker = WalkBuilder::new(root)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(false)
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
        if path
            .components()
            .any(|c| c.as_os_str() == ".git" || c.as_os_str() == "target")
        {
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let path_str = path.to_string_lossy().to_string();
        let lang = crate::database::detect_language(path);

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

        // Compute embedding if it's a small source file (skip huge ones)
        if meta.len() < 500_000 && esprit_embeddings::is_available() {
            if let Ok(content) = std::fs::read_to_string(path) {
                // Truncate to first 1000 bytes for context length
                let truncated = if content.len() > 1000 {
                    &content[..1000]
                } else {
                    &content
                };
                if let Ok(Some(emb)) = esprit_embeddings::embed(truncated) {
                    let _ = esprit_vectors::store(&path_str, &emb);
                }
            }
        }

        files.push(IndexedFile {
            path: path.to_path_buf(),
            size: meta.len(),
        });
    }

    Ok(files)
}
