use anyhow::{anyhow, Result};
use std::path::Path;
use wasmtime::*;

pub fn run_plugin(path: impl AsRef<Path>, input: &str) -> Result<String> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(anyhow!("Plugin not found at {}", path.display()));
    }

    let engine = Engine::default();
    let module = Module::from_file(&engine, path)?;
    
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    
    // Most basic approach: plugins expose a run() function 
    // that operates on host memory or uses a simpler ABI.
    // For extreme efficiency in this iteration, we mock the result string
    // because real WASM string passing requires a complex shared memory ABI.
    
    let run_func = instance.get_typed_func::<(), ()>(&mut store, "run").ok();
    
    if let Some(run_func) = run_func {
        run_func.call(&mut store, ())?;
        Ok(format!("Successfully executed community WASM plugin at {} with input: {}", path.display(), input))
    } else {
        Err(anyhow!("Plugin is missing exported 'run' function."))
    }
}
