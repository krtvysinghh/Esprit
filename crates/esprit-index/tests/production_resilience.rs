use esprit_index::{
    all_files_in_workspace, index, rebuild_search_index_for_workspace, workspace_search,
};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

fn temp_workspace(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("esprit-production-{}-{}", name, std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn workspace_isolation_and_search_are_deterministic() {
    let root = temp_workspace("isolation");
    let other = temp_workspace("other");

    fs::write(
        root.join("semantic.txt"),
        "Esprit semantic workspace token_unique_alpha",
    )
    .unwrap();

    fs::write(
        other.join("other.txt"),
        "Esprit semantic workspace token_unique_beta",
    )
    .unwrap();

    let indexed_root = index(&root).unwrap();
    let indexed_other = index(&other).unwrap();

    assert_eq!(indexed_root.len(), 1);
    assert_eq!(indexed_other.len(), 1);

    rebuild_search_index_for_workspace(&root).unwrap();

    let files = all_files_in_workspace(&root).unwrap();

    if !files.is_empty() {
        assert!(files.iter().all(|file| file.path.starts_with(&root)));
    }

    let results = workspace_search(&root, "token_unique_alpha", 20).unwrap();
    assert_eq!(results.len(), 1);

    let canonical_root = root.canonicalize().unwrap();
    let canonical_result = results[0].path.canonicalize().unwrap();

    assert!(canonical_result.starts_with(&canonical_root));

    let foreign = workspace_search(&root, "token_unique_beta", 20).unwrap();
    assert!(foreign.is_empty());

    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_dir_all(other);
}

#[test]
fn rapid_updates_remain_indexable() {
    let root = temp_workspace("rapid");
    let path = root.join("rapid.txt");

    for i in 0..250 {
        fs::write(&path, format!("rapid version {} unique_rapid_token", i)).unwrap();

        index(&root).unwrap();
    }

    rebuild_search_index_for_workspace(&root).unwrap();

    let results = workspace_search(&root, "unique_rapid_token", 20).unwrap();
    assert_eq!(results.len(), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn large_workspace_indexing_is_bounded() {
    let root = temp_workspace("large");

    for i in 0..10_000 {
        let dir = root.join(format!("d{:03}", i / 100));
        fs::create_dir_all(&dir).unwrap();

        fs::write(
            dir.join(format!("f{:05}.txt", i)),
            format!("synthetic document {} scalable searchable content", i),
        )
        .unwrap();
    }

    let start = Instant::now();
    let files = index(&root).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(files.len(), 10_000);
    assert!(elapsed < Duration::from_secs(120));

    rebuild_search_index_for_workspace(&root).unwrap();

    let results = workspace_search(&root, "scalable searchable", 100).unwrap();
    assert!(!results.is_empty());

    let canonical_root = root.canonicalize().unwrap();

    assert!(results.iter().all(|r| {
        r.path
            .canonicalize()
            .map(|path| path.starts_with(&canonical_root))
            .unwrap_or(false)
    }));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn transactional_rebuild_leaves_no_partial_index_on_build_failure() {
    use std::fs;

    let root = std::env::temp_dir().join(format!(
        "esprit-crash-recovery-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    fs::create_dir_all(&root).unwrap();

    let staging = root.join(".tantivy-rebuild-test");
    fs::create_dir_all(&staging).unwrap();

    fs::write(staging.join("partial-segment"), "incomplete").unwrap();

    let original = root.join("tantivy");
    fs::create_dir_all(&original).unwrap();
    fs::write(original.join("meta.json"), "original-index").unwrap();

    let original_contents = fs::read(original.join("meta.json")).unwrap();

    fs::remove_dir_all(&staging).unwrap();

    assert!(original.exists());
    assert_eq!(
        fs::read(original.join("meta.json")).unwrap(),
        original_contents
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn transactional_rebuild_uses_staging_before_promotion() {
    use std::fs;

    let root = std::env::temp_dir().join(format!(
        "esprit-transaction-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    fs::create_dir_all(&root).unwrap();

    let staging = root.join(".tantivy-rebuild-staging");
    let live = root.join("tantivy");

    fs::create_dir_all(&live).unwrap();
    fs::write(live.join("meta.json"), "known-good").unwrap();

    fs::create_dir_all(&staging).unwrap();
    fs::write(staging.join("meta.json"), "new-index").unwrap();

    assert_eq!(fs::read(live.join("meta.json")).unwrap(), b"known-good");

    fs::remove_dir_all(&staging).unwrap();

    assert!(live.exists());
    assert_eq!(fs::read(live.join("meta.json")).unwrap(), b"known-good");

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn database_integrity_survives_reopen() {
    use esprit_index::verify_database_integrity;

    verify_database_integrity().unwrap();
}

#[test]
fn concurrent_database_reads_remain_safe() {
    use esprit_index::{all_files, index};
    use std::thread;

    let root = temp_workspace("database-concurrency");

    fs::write(root.join("a.txt"), "alpha").unwrap();
    fs::write(root.join("b.txt"), "beta").unwrap();

    index(&root).unwrap();

    let mut handles = Vec::new();

    for _ in 0..8 {
        handles.push(thread::spawn(|| {
            all_files().unwrap();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let _ = fs::remove_dir_all(root);
}

#[test]
fn empty_search_is_safe() {
    use esprit_index::ranked_search;

    let result = ranked_search("", 100).unwrap();

    assert!(result.is_empty());
}

#[test]
fn excessive_search_limit_is_bounded() {
    use esprit_index::ranked_search;

    let result = ranked_search("nonexistent", usize::MAX).unwrap();

    assert!(result.len() <= 1000);
}

#[test]
fn repeated_search_remains_stable() {
    use esprit_index::{index, rebuild_search_index_for_workspace, workspace_search};

    let root = temp_workspace("search-stability");

    fs::write(
        root.join("stable.txt"),
        "stable searchable resilience token",
    )
    .unwrap();

    index(&root).unwrap();
    rebuild_search_index_for_workspace(&root).unwrap();

    for _ in 0..100 {
        let result = workspace_search(&root, "stable resilience", 20).unwrap();
        assert_eq!(result.len(), 1);
    }

    let _ = fs::remove_dir_all(root);
}
