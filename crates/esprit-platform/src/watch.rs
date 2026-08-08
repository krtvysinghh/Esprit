use anyhow::Result;
use esprit_index::{delete_file, insert_file, rename_file, update_file};
use notify::{
    event::{ModifyKind, RemoveKind, RenameMode},
    recommended_watcher, Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::{Duration, Instant},
};

#[derive(Clone)]
enum Pending {
    Insert(PathBuf),
    Update(PathBuf),
    Delete(PathBuf),
    Rename(PathBuf, PathBuf),
}

fn ignored(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()).map(|n| n.starts_with('.')).unwrap_or(false)
}

pub fn watch(root: impl AsRef<Path>) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(move |e: notify::Result<Event>| {
        let _ = tx.send(e);
    })?;

    watcher.watch(root.as_ref(), RecursiveMode::Recursive)?;

    let mut pending: HashMap<PathBuf, (Pending, Instant)> = HashMap::new();

    loop {
        while let Ok(event) = rx.try_recv() {
            let event = match event {
                Ok(e) => e,
                Err(_) => continue,
            };

            match event.kind {
                EventKind::Create(_) => {
                    for p in event.paths {
                        if !ignored(&p) {
                            pending.insert(p.clone(), (Pending::Insert(p), Instant::now()));
                        }
                    }
                }

                EventKind::Modify(ModifyKind::Data(_)) | EventKind::Modify(ModifyKind::Any) => {
                    for p in event.paths {
                        if !ignored(&p) {
                            pending.insert(p.clone(), (Pending::Update(p), Instant::now()));
                        }
                    }
                }

                EventKind::Remove(RemoveKind::File) | EventKind::Remove(RemoveKind::Any) => {
                    for p in event.paths {
                        if !ignored(&p) {
                            pending.insert(p.clone(), (Pending::Delete(p), Instant::now()));
                        }
                    }
                }

                EventKind::Modify(ModifyKind::Name(RenameMode::Both)) if event.paths.len() == 2 => {
                    let a = event.paths[0].clone();
                    let b = event.paths[1].clone();

                    if !ignored(&a) && !ignored(&b) {
                        pending.insert(b.clone(), (Pending::Rename(a, b), Instant::now()));
                    }
                }

                _ => {}
            }
        }

        let now = Instant::now();

        let ready: Vec<_> = pending
            .iter()
            .filter(|(_, (_, t))| now.duration_since(*t) >= Duration::from_millis(100))
            .map(|(k, _)| k.clone())
            .collect();

        for key in ready {
            if let Some((ev, _)) = pending.remove(&key) {
                match ev {
                    Pending::Insert(p) => {
                        let _ = insert_file(&p);
                        println!("+ {}", p.display());
                    }

                    Pending::Update(p) => {
                        let _ = update_file(&p);
                        println!("~ {}", p.display());
                    }

                    Pending::Delete(p) => {
                        let _ = delete_file(&p);
                        println!("- {}", p.display());
                    }

                    Pending::Rename(a, b) => {
                        let _ = rename_file(&a, &b);
                        println!("R {} -> {}", a.display(), b.display());
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(25));
    }
}
