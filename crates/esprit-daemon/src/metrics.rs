use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct Metrics {
    started: Instant,
    restarts: AtomicU64,
    requests: AtomicU64,
    errors: AtomicU64,
}

impl Metrics {
    pub const fn new() -> Self {
        Self {
            started: Instant::now(),
            restarts: AtomicU64::new(0),
            requests: AtomicU64::new(0),
            errors: AtomicU64::new(0),
        }
    }

    pub fn uptime(&self) -> u64 {
        self.started.elapsed().as_secs()
    }

    pub fn restart_count(&self) -> u64 {
        self.restarts.load(Ordering::Relaxed)
    }

    pub fn request_count(&self) -> u64 {
        self.requests.load(Ordering::Relaxed)
    }

    pub fn error_count(&self) -> u64 {
        self.errors.load(Ordering::Relaxed)
    }

    pub fn record_restart(&self) {
        self.restarts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_request(&self) {
        self.requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}

pub static METRICS: Metrics = Metrics::new();
