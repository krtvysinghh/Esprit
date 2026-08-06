use grep_regex::RegexMatcher;
use grep_searcher::{Searcher, sinks::UTF8};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

pub fn search(pattern: &str, root: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build()
        .par_bridge()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().contains(pattern))
                .unwrap_or(false)
        })
        .collect()
}

pub fn search_contents(pattern: &str, root: impl AsRef<Path>) -> Vec<(PathBuf, usize, String)> {
    let matcher = RegexMatcher::new_line_matcher(pattern).unwrap();

    WalkBuilder::new(root)
        .hidden(false)
        .build()
        .par_bridge()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .flat_map(|path| {
            let mut out = Vec::new();

            let mut searcher = Searcher::new();

            let _ = searcher.search_path(
                &matcher,
                &path,
                UTF8(|ln, line| {
                    out.push((path.clone(), ln as usize, line.to_string()));
                    Ok(true)
                }),
            );

            out
        })
        .collect()
}
