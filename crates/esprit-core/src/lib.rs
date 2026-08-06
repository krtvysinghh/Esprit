pub const APP_NAME: &str = "Esprit";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn banner() -> String {
    format!("{APP_NAME} {VERSION}")
}
