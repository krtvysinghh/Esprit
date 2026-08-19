use crate::metrics::METRICS;

pub struct Health {
    pub status: &'static str,
    pub uptime: u64,
    pub restarts: u64,
}

pub fn status() -> Health {
    Health {
        status: "healthy",
        uptime: METRICS.uptime(),
        restarts: METRICS.restart_count(),
    }
}
