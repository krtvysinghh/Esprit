#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
use anyhow::Result;

pub trait Stage<T> {
    fn run(&self, input: T) -> Result<T>;
}

pub struct Pipeline<T> {
    stages: Vec<Box<dyn Stage<T>>>,
}

impl<T: 'static> Default for Pipeline<T> {
    fn default() -> Self {
        Self { stages: Vec::new() }
    }
}

impl<T: 'static> Pipeline<T> {
    pub fn add<S: Stage<T> + 'static>(&mut self, s: S) {
        self.stages.push(Box::new(s));
    }

    pub fn execute(&self, mut value: T) -> Result<T> {
        for s in &self.stages {
            value = s.run(value)?;
        }
        Ok(value)
    }
}
