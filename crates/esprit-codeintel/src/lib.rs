use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: String,
    pub file: String,
    pub line: usize,
}

pub fn index(root: impl AsRef<Path>) -> Result<Vec<Symbol>> {
    let re = Regex::new(r"(fn|struct|enum|trait|impl)\s+([A-Za-z0-9_]+)")?;
    let mut out = Vec::new();

    for e in WalkDir::new(root) {
        let e = e?;
        if !e.file_type().is_file() {
            continue;
        }

        if let Ok(text) = fs::read_to_string(e.path()) {
            for (i, l) in text.lines().enumerate() {
                if let Some(c) = re.captures(l) {
                    out.push(Symbol {
                        kind: c[1].to_string(),
                        name: c[2].to_string(),
                        file: e.path().display().to_string(),
                        line: i + 1,
                    });
                }
            }
        }
    }

    Ok(out)
}
