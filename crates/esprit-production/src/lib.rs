use anyhow::Result;
use tracing_subscriber::FmtSubscriber;

pub fn init() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_level(true)
        .compact()
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);

    Ok(())
}
