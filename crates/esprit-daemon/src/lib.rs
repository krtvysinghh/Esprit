use anyhow::Result;
use crossbeam_channel::unbounded;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use std::path::Path;

pub fn run(root: impl AsRef<Path>) -> Result<()> {
    let (tx, rx) = unbounded();

    let mut watcher = recommended_watcher(move |e| {
        let _ = tx.send(e);
    })?;

    watcher.watch(root.as_ref(), RecursiveMode::Recursive)?;

    loop {
        let _ = rx.recv()?;
    }
}
