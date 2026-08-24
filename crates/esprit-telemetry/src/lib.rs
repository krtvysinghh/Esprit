use anyhow::Result;

/// Initialise structured logging for Esprit.
///
/// Reads `RUST_LOG` for level control. Safe to call multiple times — only
/// the first call has effect.
pub fn init() -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};
    let _ = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .compact()
        .try_init();
    Ok(())
}
