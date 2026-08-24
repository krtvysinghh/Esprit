use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// Move files into extension-named subdirectories under `root`.
///
/// If `dry_run` is `true`, no files are moved; the planned operations are
/// returned instead so the caller can display them.
pub fn organize(root: impl AsRef<Path>) -> Result<Vec<MoveOp>> {
    organize_with_options(root, false)
}

/// Dry-run: return planned moves without touching the filesystem.
pub fn organize_dry_run(root: impl AsRef<Path>) -> Result<Vec<MoveOp>> {
    organize_with_options(root, true)
}

#[derive(Debug, Clone)]
pub struct MoveOp {
    pub from: PathBuf,
    pub to: PathBuf,
}

fn organize_with_options(root: impl AsRef<Path>, dry_run: bool) -> Result<Vec<MoveOp>> {
    let root = root.as_ref();
    let mut by_ext: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(root).max_depth(1) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let ext = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("misc")
            .to_lowercase();
        by_ext
            .entry(ext)
            .or_default()
            .push(entry.path().to_path_buf());
    }

    let mut ops = Vec::new();
    for (ext, files) in by_ext {
        let target_dir = root.join(&ext);
        if !dry_run {
            fs::create_dir_all(&target_dir)?;
        }
        for file in files {
            let fname = file.file_name().unwrap_or_default();
            let dest = target_dir.join(fname);
            if file == dest {
                continue;
            }
            ops.push(MoveOp {
                from: file.clone(),
                to: dest.clone(),
            });
            if !dry_run {
                fs::rename(&file, &dest)?;
            }
        }
    }

    ops.sort_by(|a, b| a.from.cmp(&b.from));
    Ok(ops)
}
