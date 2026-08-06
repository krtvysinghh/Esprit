use anyhow::Result;
use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::path::{Path, PathBuf};

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
        let walker = WalkBuilder::new(options.root)
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build();

        if options.regex {
            let re = Regex::new(&options.pattern)?;

            let files = walker
                .par_bridge()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.into_path();

                    let name = path.file_name()?.to_string_lossy();

                    if re.is_match(&name) { Some(SearchResult { path }) } else { None }
                })
                .collect();

            return Ok(files);
        }

        let mut builder = GlobSetBuilder::new();

        builder.add(Glob::new(&options.pattern)?);

        let matcher = builder.build()?;

        let files = walker
            .par_bridge()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.into_path();

                if matcher.is_match(&path) { Some(SearchResult { path }) } else { None }
            })
            .collect();

        Ok(files)
    }
}
