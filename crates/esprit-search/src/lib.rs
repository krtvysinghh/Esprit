use anyhow::Result;

pub use esprit_index::search;

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub pattern: String,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn run(options: SearchOptions) -> Result<Vec<SearchResult>> {
        Ok(search(&options.pattern)?.into_iter().map(|p| SearchResult { path: p }).collect())
    }
}
