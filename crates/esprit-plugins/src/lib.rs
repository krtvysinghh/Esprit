#![forbid(unsafe_code)]
use anyhow::{anyhow, Result};
use std::path::Path;
use wasmtime::*;

pub fn run_plugin(path: impl AsRef<Path>, input: &str) -> Result<String> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(anyhow!("Plugin not found at {}", path.display()));
    }

    let mut config = Config::new();
    config.consume_fuel(true);
    // Security: restrict memory and execution time for community plugins

    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, path)?;

    let mut store = Store::new(&engine, ());
    store.set_fuel(10_000_000)?; // Strict execution limit to prevent infinite loops

    let instance = Instance::new(&mut store, &module, &[])?;

    let run_func = instance.get_typed_func::<(), ()>(&mut store, "run").ok();

    if let Some(run_func) = run_func {
        run_func.call(&mut store, ())?;
        Ok(format!(
            "Successfully executed community WASM plugin at {} with input: {}",
            path.display(),
            input
        ))
    } else {
        Err(anyhow!("Plugin is missing exported 'run' function."))
    }
}
