use esprit_filesystem::{duplicates, FolderStats};
use std::fs;

#[test]
fn duplicate_scan_is_stable() {
    let root = std::env::temp_dir().join("esprit-fs-duplicate-test");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    fs::write(root.join("a.txt"), "same").unwrap();
    fs::write(root.join("b.txt"), "same").unwrap();

    let result = duplicates(&root).unwrap();

    assert_eq!(result.len(), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn folder_stats_handles_large_empty_tree() {
    let root = std::env::temp_dir().join("esprit-fs-stats-test");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("nested")).unwrap();

    let stats = FolderStats::scan(&root).unwrap();

    assert!(stats.directories >= 1);

    let _ = fs::remove_dir_all(root);
}
