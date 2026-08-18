use std::path::Path;
use std::fs;

pub fn check_tantivy_dir() -> bool {
    let path = Path::new(".esprit/index");
    if !path.exists() {
        fs::create_dir_all(path).is_ok()
    } else {
        true
    }
}

pub fn check_sqlite_store() -> bool {
    let path = Path::new(".esprit/db");
    if !path.exists() {
        fs::create_dir_all(path).is_ok()
    } else {
        true
    }
}
