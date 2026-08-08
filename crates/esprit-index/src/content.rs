use std::{fs, path::Path};

const MAX_SIZE: u64 = 2 * 1024 * 1024;

const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "py", "md", "txt", "toml", "json", "yaml", "yml", "html", "css", "scss", "sql", "sh",
    "zsh", "xml", "js", "ts", "tsx", "jsx", "go", "java", "kt", "c", "cpp", "h", "hpp",
];

pub fn extract(path: &Path) -> String {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return String::new(),
    };

    if meta.len() > MAX_SIZE {
        return String::new();
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if !TEXT_EXTENSIONS.contains(&ext.as_str()) {
        return String::new();
    }

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };

    if bytes.contains(&0) {
        return String::new();
    }

    String::from_utf8(bytes).unwrap_or_default()
}
