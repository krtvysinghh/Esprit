use anyhow::{anyhow, Result};

/// The plugin contract. Plugins are `Send + Sync` so they can live behind a
/// shared reference across threads.
pub trait Plugin: Send + Sync {
    /// Unique identifier for this plugin (e.g. `"my-plugin"`).
    fn name(&self) -> &'static str;
    /// Human-readable description shown by `esprit plugins list`.
    fn description(&self) -> &'static str {
        ""
    }
    /// Execute the plugin with the given text input and return a response.
    fn run(&self, input: &str) -> Result<String>;
}

/// Runtime registry of loaded plugins.
#[derive(Default)]
pub struct Registry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// List the names of all registered plugins.
    pub fn list(&self) -> Vec<&'static str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }

    /// Run a plugin by name.
    pub fn run(&self, name: &str, input: &str) -> Result<String> {
        self.plugins
            .iter()
            .find(|p| p.name() == name)
            .ok_or_else(|| anyhow!("plugin '{}' not found — available: {:?}", name, self.list()))?
            .run(input)
    }
}
