#[test]
fn workspace_integrity() {
    assert!(std::path::Path::new("Cargo.toml").exists());
}
