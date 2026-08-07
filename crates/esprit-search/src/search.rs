use anyhow::Result;
use esprit_index::all_files;
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub root: PathBuf,
    pub pattern: String,
    pub regex: bool,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn run(options: SearchOptions) -> Result<Vec<SearchResult>> {
        let files = all_files()?;

        if options.regex {
            let re = Regex::new(&options.pattern)?;

            return Ok(files
                .into_iter()
                .filter(|file| {
                    file.path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| re.is_match(n))
                        .unwrap_or(false)
                })
                .map(|file| SearchResult { path: file.path })
                .collect());
        }

        let pattern = options.pattern.to_lowercase();

        Ok(files
            .into_iter()
            .filter(|file| {
                file.path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.to_lowercase().contains(&pattern))
                    .unwrap_or(false)
            })
            .map(|file| SearchResult { path: file.path })
            .collect())
    }
}
