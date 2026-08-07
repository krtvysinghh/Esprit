use anyhow::Result;
use std::path::Path;

pub struct IndexEngine;

impl IndexEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, _: impl AsRef<Path>) -> Result<()> {
        Ok(())
    }

    pub fn update(&self, _: impl AsRef<Path>) -> Result<()> {
        Ok(())
    }

    pub fn remove(&self, _: impl AsRef<Path>) -> Result<()> {
        Ok(())
    }

    pub fn optimize(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for IndexEngine {
    fn default() -> Self {
        Self::new()
    }
}
