use anyhow::Result;
use std::time::Instant;

pub struct Bench {
    pub name: &'static str,
    pub elapsed_ms: u128,
}

pub fn measure<F: FnOnce() -> Result<()>>(name: &'static str, f: F) -> Result<Bench> {
    let now = Instant::now();
    f()?;
    Ok(Bench { name, elapsed_ms: now.elapsed().as_millis() })
}
