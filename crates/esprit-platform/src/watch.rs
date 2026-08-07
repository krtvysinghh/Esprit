use anyhow::Result;
use esprit_index::{delete_file, insert_file, rename_file, update_file};
use notify::{
    Event, EventKind, RecursiveMode, Watcher,
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    recommended_watcher,
};
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
            Ok(event) => match event.kind {
                EventKind::Create(CreateKind::File) | EventKind::Create(CreateKind::Any) => {
                    for path in &event.paths {
                        let _ = insert_file(path);
                        println!("+ {}", path.display());
                    }
                }

                EventKind::Modify(ModifyKind::Data(_))
                | EventKind::Modify(ModifyKind::Metadata(_))
                | EventKind::Modify(ModifyKind::Any) => {
                    for path in &event.paths {
                        let _ = update_file(path);
                        println!("~ {}", path.display());
                    }
                }

                EventKind::Remove(RemoveKind::File) | EventKind::Remove(RemoveKind::Any) => {
                    for path in &event.paths {
                        let _ = delete_file(path);
                        println!("- {}", path.display());
                    }
                }

                EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                    if event.paths.len() == 2 {
                        let _ = rename_file(&event.paths[0], &event.paths[1]);

                        println!("R {} -> {}", event.paths[0].display(), event.paths[1].display());
                    }
                }

                _ => {}
            },

            Err(err) => {
                eprintln!("{err}");
            }
        }
    }
}
