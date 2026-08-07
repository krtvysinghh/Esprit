use std::{fs, path::Path};

const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "toml", "md", "txt", "json", "yaml", "yml", "js", "ts", "tsx", "jsx", "py", "go", "java",
    "kt", "c", "cpp", "h", "hpp", "html", "css", "scss", "sql", "sh", "zsh", "xml",
];

pub fn extract(path: &Path) -> String {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();

    if !TEXT_EXTENSIONS.contains(&ext.as_str()) {
        return String::new();
    }

    fs::read_to_string(path).unwrap_or_default()
}
