use sysinfo::System;

#[derive(Debug)]
pub struct PlatformInfo {
    pub os: String,
    pub cpu_cores: usize,
    pub memory_gb: f64,
}

pub fn current() -> PlatformInfo {
    let mut system = System::new_all();
    system.refresh_all();

    let os = System::name().unwrap_or_else(|| "Unknown".to_string());

    let cpu_cores = system.cpus().len();

    let memory_gb = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    PlatformInfo { os, cpu_cores, memory_gb }
}
