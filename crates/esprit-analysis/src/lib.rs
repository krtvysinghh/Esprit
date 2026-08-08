use anyhow::Result;
use esprit_codeintel::{index, Symbol};
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};
use walkdir::WalkDir;

pub fn references(root: impl AsRef<Path>) -> Result<HashMap<String, usize>> {
    let symbols = index(&root)?;
    let mut map = HashMap::new();

    for Symbol { name, .. } in symbols {
        map.insert(name.clone(), 0usize);
    }

    for e in WalkDir::new(root) {
        let e = e?;
        if !e.file_type().is_file() {
            continue;
        }

        if let Ok(text) = fs::read_to_string(e.path()) {
            for name in map.clone().keys() {
                let re = Regex::new(&format!(r"\b{}\b", regex::escape(name)))?;
                let c = re.find_iter(&text).count();
                *map.entry(name.clone()).or_default() += c;
            }
        }
    }

    Ok(map)
}
