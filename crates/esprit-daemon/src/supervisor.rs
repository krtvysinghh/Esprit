use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

pub struct Supervisor {
    max_restarts: u32,
}

impl Supervisor {
    pub fn new(max_restarts: u32) -> Self {
        Self { max_restarts }
    }

    pub async fn run<F, Fut>(&self, mut task: F)
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<()>>,
    {
        let mut attempts = 0;

        loop {
            match task().await {
                Ok(_) => {
                    info!("service exited normally");
                    break;
                }

                Err(err) => {
                    attempts += 1;

                    error!(
                        error = %err,
                        attempt = attempts,
                        "service crashed"
                    );

                    if attempts >= self.max_restarts {
                        error!("maximum restart limit reached");
                        break;
                    }

                    let delay = Duration::from_secs(2u64.pow(attempts.min(5)));

                    info!(?delay, "restarting service");

                    sleep(delay).await;
                }
            }
        }
    }
}
