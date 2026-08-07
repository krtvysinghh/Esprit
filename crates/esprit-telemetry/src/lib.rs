use anyhow::Result;

pub fn init() -> Result<()> {
    let _ = tracing_subscriber::fmt().with_target(false).compact().try_init();
    Ok(())
}
