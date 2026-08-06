pub mod stats;

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

const IMAGES: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg", "heic"];
const VIDEOS: &[&str] = &["mp4", "mkv", "mov", "avi", "webm"];
const DOCUMENTS: &[&str] = &["pdf", "doc", "docx", "ppt", "pptx", "xls", "xlsx", "txt"];
const ARCHIVES: &[&str] = &["zip", "7z", "rar", "tar", "gz"];
const AUDIO: &[&str] = &["mp3", "wav", "flac", "m4a"];
const CODE: &[&str] = &[
    "rs", "py", "js", "ts", "tsx", "jsx", "go", "java", "kt", "c", "cpp", "h", "hpp", "swift",
    "toml", "json", "yaml", "yml", "xml", "html", "css", "scss", "md", "sql", "sh", "zsh",
];

fn unique_path(dir: &Path, file: &Path) -> PathBuf {
    let stem = file.file_stem().unwrap().to_string_lossy();
    let ext = file
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    let mut target = dir.join(file.file_name().unwrap());

    if !target.exists() {
        return target;
    }

    let mut i = 1;

    loop {
        target = dir.join(format!("{stem} ({i}){ext}"));

        if !target.exists() {
            return target;
        }

        i += 1;
    }
}

pub fn organize(root: impl AsRef<Path>) -> Result<()> {
    let root = root.as_ref();

    let groups: &[(&str, &[&str])] = &[
        ("Images", IMAGES),
        ("Videos", VIDEOS),
        ("Documents", DOCUMENTS),
        ("Archives", ARCHIVES),
        ("Audio", AUDIO),
        ("Code", CODE),
    ];

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        for (folder, exts) in groups {
            if exts.contains(&ext.as_str()) {
                let dst = root.join(folder);
                fs::create_dir_all(&dst)?;
                let target = unique_path(&dst, &path);
                fs::rename(&path, target)?;
                break;
            }
        }
    }

    Ok(())
}
