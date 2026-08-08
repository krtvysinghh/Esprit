use std::process::Command;
use sysinfo::System;

#[derive(Debug)]
pub struct DoctorReport {
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub cpu: String,
    pub cpu_cores: usize,
    pub ram_gb: f64,
    pub rust: bool,
    pub cargo: bool,
    pub git: bool,
    pub ollama: bool,
}

fn exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn doctor() -> DoctorReport {
    let mut sys = System::new_all();
    sys.refresh_all();

    DoctorReport {
        os: System::name().unwrap_or_default(),
        kernel: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        cpu: sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default(),
        cpu_cores: sys.cpus().len(),
        ram_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        rust: exists("rustc"),
        cargo: exists("cargo"),
        git: exists("git"),
        ollama: exists("ollama"),
    }
}
