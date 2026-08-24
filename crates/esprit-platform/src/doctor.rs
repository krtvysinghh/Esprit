use anyhow::Result;
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
    pub rust_version: Option<String>,
    pub cargo: bool,
    pub git: bool,
    pub git_version: Option<String>,
    pub ollama: bool,
    pub ollama_version: Option<String>,
}

/// Run a command and capture its stdout, returning `None` on failure.
fn capture(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Check if a command exists in PATH in a cross-platform way.
fn exists(cmd: &str) -> bool {
    #[cfg(windows)]
    {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(windows))]
    {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

pub fn doctor() -> DoctorReport {
    let mut sys = System::new_all();
    sys.refresh_all();

    let rust = exists("rustc");
    let rust_version = capture("rustc", &["--version"])
        .and_then(|s| s.split_whitespace().nth(1).map(String::from));

    let git = exists("git");
    let git_version = capture("git", &["--version"])
        .and_then(|s| s.split_whitespace().nth(2).map(String::from));

    let ollama = exists("ollama");
    let ollama_version = capture("ollama", &["--version"])
        .and_then(|s| s.split_whitespace().last().map(String::from));

    DoctorReport {
        os: System::name().unwrap_or_else(|| "Unknown".into()),
        kernel: System::kernel_version().unwrap_or_else(|| "Unknown".into()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".into()),
        cpu: sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".into()),
        cpu_cores: sys.cpus().len(),
        ram_gb: sys.total_memory() as f64 / 1_073_741_824.0,
        rust,
        rust_version,
        cargo: exists("cargo"),
        git,
        git_version,
        ollama,
        ollama_version,
    }
}

/// Check if Ollama HTTP API is reachable (separate from binary detection).
#[allow(dead_code)]
pub fn ollama_reachable() -> Result<bool> {
    let url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    Ok(reqwest::blocking::get(format!("{url}/api/tags"))
        .map(|r| r.status().is_success())
        .unwrap_or(false))
}
