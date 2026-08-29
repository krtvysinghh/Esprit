#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
use anyhow::Result;

pub fn release() -> Result<()> {
    println!("Packaging Esprit...");
    Ok(())
}
