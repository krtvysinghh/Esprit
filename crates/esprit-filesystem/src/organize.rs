use anyhow::{bail, Context, Result};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// Organizes files in `root` into extension-named directories.
///
/// This operation is intentionally conservative:
/// - only regular files directly inside `root` are considered;
/// - hidden files are ignored;
/// - files without an extension are ignored;
/// - existing directories are never moved;
/// - destination names are made collision-safe;
/// - a file is never overwritten;
/// - symlinks are ignored.
///
/// Example:
/// `root/report.pdf` -> `root/pdf/report.pdf`
pub fn organize(root: impl AsRef<Path>) -> Result<Vec<(PathBuf, PathBuf)>> {
    let root = root.as_ref();

    if !root.exists() {
        bail!("organization root does not exist: {}", root.display());
    }

    if !root.is_dir() {
        bail!("organization root is not a directory: {}", root.display());
    }

    let mut entries = fs::read_dir(root)
        .with_context(|| format!("failed to read organization root: {}", root.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate {}", root.display()))?;

    entries.sort_by_key(|entry| entry.file_name());

    let mut planned = Vec::new();
    let mut reserved = HashSet::new();

    for entry in entries {
        let path = entry.path();

        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                bail!("failed to inspect {}: {error}", path.display());
            }
        };

        if !metadata.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with('.') {
            continue;
        }

        let extension = match path.extension().and_then(|value| value.to_str()) {
            Some(extension) if !extension.is_empty() => extension.to_ascii_lowercase(),
            _ => continue,
        };

        let destination_dir = root.join(&extension);
        let destination = unique_destination(&destination_dir, &path, &mut reserved)?;

        planned.push((path, destination));
    }

    for (source, destination) in &planned {
        fs::create_dir_all(
            destination
                .parent()
                .context("organization destination has no parent directory")?,
        )
        .with_context(|| {
            format!(
                "failed to create destination directory for {}",
                destination.display()
            )
        })?;

        fs::rename(source, destination).with_context(|| {
            format!(
                "failed to move {} to {}",
                source.display(),
                destination.display()
            )
        })?;
    }

    Ok(planned)
}

fn unique_destination(
    directory: &Path,
    source: &Path,
    reserved: &mut HashSet<PathBuf>,
) -> Result<PathBuf> {
    let file_name = source
        .file_name()
        .context("file has no filename")?
        .to_owned();

    let candidate = directory.join(&file_name);

    if !candidate.exists() && reserved.insert(candidate.clone()) {
        return Ok(candidate);
    }

    let stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("file");

    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    for index in 1u64.. {
        let name = if extension.is_empty() {
            format!("{stem}-{index}")
        } else {
            format!("{stem}-{index}.{extension}")
        };

        let candidate = directory.join(name);

        if !candidate.exists() && reserved.insert(candidate.clone()) {
            return Ok(candidate);
        }
    }

    unreachable!("u64 destination namespace exhausted")
}

#[cfg(test)]
mod tests {
    use super::organize;
    use std::{
        fs,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_root(name: &str) -> std::path::PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("esprit-organize-{name}-{suffix}"))
    }

    fn write(path: &Path, contents: &str) {
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn organizes_files_by_extension() {
        let root = temp_root("basic");
        fs::create_dir_all(&root).unwrap();

        write(&root.join("photo.JPG"), "image");
        write(&root.join("notes.txt"), "text");

        let moved = organize(&root).unwrap();

        assert_eq!(moved.len(), 2);
        assert!(root.join("jpg/photo.JPG").is_file());
        assert!(root.join("txt/notes.txt").is_file());
        assert!(!root.join("photo.JPG").exists());
        assert!(!root.join("notes.txt").exists());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ignores_hidden_and_extensionless_files() {
        let root = temp_root("ignored");
        fs::create_dir_all(&root).unwrap();

        write(&root.join(".env"), "secret");
        write(&root.join("README"), "readme");
        write(&root.join("file.txt"), "text");

        let moved = organize(&root).unwrap();

        assert_eq!(moved.len(), 1);
        assert!(root.join(".env").is_file());
        assert!(root.join("README").is_file());
        assert!(root.join("txt/file.txt").is_file());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn never_overwrites_existing_destination() {
        let root = temp_root("collision");
        let destination = root.join("txt");
        fs::create_dir_all(&destination).unwrap();

        write(&root.join("file.txt"), "new");
        write(&destination.join("file.txt"), "existing");

        let moved = organize(&root).unwrap();

        assert_eq!(moved.len(), 1);
        assert_eq!(
            fs::read_to_string(destination.join("file.txt")).unwrap(),
            "existing"
        );
        assert_eq!(
            fs::read_to_string(destination.join("file-1.txt")).unwrap(),
            "new"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ignores_directories_and_symlinks() {
        let root = temp_root("safe");
        fs::create_dir_all(root.join("existing")).unwrap();

        write(&root.join("file.txt"), "text");

        #[cfg(unix)]
        std::os::unix::fs::symlink(root.join("file.txt"), root.join("link.txt")).unwrap();

        let moved = organize(&root).unwrap();

        assert_eq!(moved.len(), 1);
        assert!(root.join("existing").is_dir());
        assert!(root.join("txt/file.txt").is_file());

        #[cfg(unix)]
        assert!(!root.join("link.txt").exists());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_missing_root() {
        let root = temp_root("missing");
        assert!(organize(&root).is_err());
    }

    #[test]
    fn rejects_file_root() {
        let root = temp_root("file-root");
        write(&root, "not a directory");

        assert!(organize(&root).is_err());

        fs::remove_file(root).unwrap();
    }
}
