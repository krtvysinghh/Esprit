use anyhow::Result;
use crossbeam_channel::{unbounded, RecvTimeoutError};
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

const RECOVERY_DELAY: Duration = Duration::from_millis(250);
const WATCH_POLL_INTERVAL: Duration = Duration::from_millis(100);

fn watch_once<F>(root: &Path, stop: &AtomicBool, on_event: &F) -> Result<()>
where
    F: Fn(Event) + Send + Sync,
{
    let (tx, rx) = unbounded::<notify::Result<Event>>();

    let mut watcher = recommended_watcher(move |event| {
        let _ = tx.send(event);
    })?;

    watcher.watch(root, RecursiveMode::Recursive)?;

    loop {
        if stop.load(Ordering::Relaxed) {
            return Ok(());
        }

        match rx.recv_timeout(WATCH_POLL_INTERVAL) {
            Ok(Ok(event)) => {
                on_event(event);
            }

            Ok(Err(error)) => {
                anyhow::bail!("filesystem watcher reported an error: {error}");
            }

            Err(RecvTimeoutError::Timeout) => {
                if !root.exists() {
                    anyhow::bail!("watched root disappeared: {}", root.display());
                }
            }

            Err(RecvTimeoutError::Disconnected) => {
                anyhow::bail!("filesystem watcher channel disconnected");
            }
        }
    }
}

pub fn run(root: impl AsRef<Path>) -> Result<()> {
    let root = PathBuf::from(root.as_ref());
    let stop = Arc::new(AtomicBool::new(false));

    run_with_handler(root, stop, |_| {})
}

pub fn run_with_handler<F>(root: impl AsRef<Path>, stop: Arc<AtomicBool>, on_event: F) -> Result<()>
where
    F: Fn(Event) + Send + Sync,
{
    let root = PathBuf::from(root.as_ref());

    if !root.exists() {
        anyhow::bail!("watch root does not exist: {}", root.display());
    }

    loop {
        if stop.load(Ordering::Relaxed) {
            return Ok(());
        }

        match watch_once(&root, &stop, &on_event) {
            Ok(()) => return Ok(()),

            Err(error) => {
                if stop.load(Ordering::Relaxed) {
                    return Ok(());
                }

                eprintln!(
                    "daemon watcher failed: {error}; restarting in {}ms...",
                    RECOVERY_DELAY.as_millis()
                );

                std::thread::sleep(RECOVERY_DELAY);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicUsize, Ordering},
        thread,
        time::{Duration, Instant},
    };

    fn test_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "esprit-daemon-{name}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ))
    }

    fn wait_until(timeout: Duration, condition: impl Fn() -> bool) -> bool {
        let start = Instant::now();

        while start.elapsed() < timeout {
            if condition() {
                return true;
            }

            thread::sleep(Duration::from_millis(25));
        }

        false
    }

    #[test]
    fn missing_root_is_rejected() {
        let root = test_root("missing");
        let _ = fs::remove_dir_all(&root);

        let stop = AtomicBool::new(false);

        let result = watch_once(&root, &stop, &|_| {});

        assert!(result.is_err());
    }

    #[test]
    fn daemon_stops_cleanly() {
        let root = test_root("stop");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = Arc::clone(&stop);
        let thread_root = root.clone();

        let handle = thread::spawn(move || run_with_handler(thread_root, thread_stop, |_| {}));

        thread::sleep(Duration::from_millis(300));
        stop.store(true, Ordering::Relaxed);

        let result = handle.join().unwrap();

        assert!(result.is_ok());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn watcher_recovers_after_root_is_recreated() {
        let root = test_root("recovery");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let stop = Arc::new(AtomicBool::new(false));
        let events = Arc::new(AtomicUsize::new(0));

        let thread_stop = Arc::clone(&stop);
        let thread_events = Arc::clone(&events);
        let thread_root = root.clone();

        let handle = thread::spawn(move || {
            run_with_handler(thread_root, thread_stop, move |_| {
                thread_events.fetch_add(1, Ordering::Relaxed);
            })
        });

        thread::sleep(Duration::from_millis(300));

        fs::remove_dir_all(&root).unwrap();

        assert!(
            wait_until(Duration::from_secs(3), || !root.exists()),
            "test root was not removed"
        );

        thread::sleep(Duration::from_millis(500));

        fs::create_dir_all(&root).unwrap();

        assert!(
            wait_until(Duration::from_secs(3), || root.exists()),
            "test root was not recreated"
        );

        let file = root.join("recovery-test.txt");
        fs::write(&file, "recovery").unwrap();

        assert!(
            wait_until(Duration::from_secs(5), || {
                events.load(Ordering::Relaxed) > 0
            }),
            "watcher did not recover after root recreation"
        );

        stop.store(true, Ordering::Relaxed);

        let result = handle.join().unwrap();

        assert!(result.is_ok());

        let _ = fs::remove_dir_all(root);
    }
}
