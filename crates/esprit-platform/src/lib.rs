#[derive(Debug, Clone, Copy)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
    Unknown,
}

pub fn current() -> Platform {
    #[cfg(target_os = "macos")]
    return Platform::MacOS;

    #[cfg(target_os = "linux")]
    return Platform::Linux;

    #[cfg(target_os = "windows")]
    return Platform::Windows;

    #[allow(unreachable_code)]
    Platform::Unknown
}
