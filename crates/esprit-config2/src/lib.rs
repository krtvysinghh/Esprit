use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Runtime {
    pub workers: usize,
    pub cache_mb: usize,
}

pub fn load() -> Result<Runtime> {
    Ok(Runtime::default())
}
