#![forbid(unsafe_code)]
use anyhow::Result;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self
    }
}

impl App {
    pub fn boot() -> Result<Self> {
        let _ = esprit_config::Config::load()?;
        Ok(Self)
    }
}
