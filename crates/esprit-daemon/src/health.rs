use crate::metrics::metrics;

pub struct Health {
    pub status: &'static str,
    pub uptime: u64,
    pub restarts: u64,
}

pub fn status() -> Health {
    Health {
        status: "healthy",
        uptime: metrics().uptime(),
        restarts: metrics().restart_count(),
    }
}
