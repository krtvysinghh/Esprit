use anyhow::Result;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn run(&self, input: &str) -> Result<String>;
}

pub struct Registry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Registry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn list(&self) -> Vec<&'static str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
}
