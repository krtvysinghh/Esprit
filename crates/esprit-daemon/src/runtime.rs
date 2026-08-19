use std::sync::Arc;
use tokio::sync::Notify;
use tracing::{info, warn};

pub struct Runtime {
    shutdown: Arc<Notify>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Notify::new()),
        }
    }

    pub async fn run(&self) {
        info!("Esprit daemon started");

        tokio::select! {

            _ = self.shutdown.notified() => {
                info!("Shutdown signal received");
            }

        }

        self.shutdown_sequence().await;
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_one();
    }

    async fn shutdown_sequence(&self) {
        info!("Stopping workers");

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        warn!("Esprit daemon stopped cleanly");
    }
}
