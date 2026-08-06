use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

pub fn search(pattern: &str, root: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).unwrap());

    let matcher = builder.build().unwrap();

    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    walker
        .par_bridge()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| matcher.is_match(p))
        .collect()
}
