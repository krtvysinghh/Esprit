use anyhow::Result;
use tracing_subscriber::FmtSubscriber;

pub fn init() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_level(true)
        .compact()
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        return Ok(());
    }

    Ok(())
}
