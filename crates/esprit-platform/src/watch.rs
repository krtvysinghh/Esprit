use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher, recommended_watcher};
use std::{path::Path, sync::mpsc::channel};

pub fn watch(root: impl AsRef<Path>) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
        tx.send(res).unwrap();
    })?;

    watcher.watch(root.as_ref(), RecursiveMode::Recursive)?;

    println!("Watching {}\n", root.as_ref().display());

    loop {
        match rx.recv()? {
            Ok(event) => {
                println!("{:?}", event.kind);

                for path in event.paths {
                    println!("  {}", path.display());
                }

                println!();
            }

            Err(err) => {
                eprintln!("{err}");
            }
        }
    }
}
