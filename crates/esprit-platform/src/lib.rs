#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
pub mod doctor;
mod watch;

pub use doctor::{doctor, DoctorReport};
pub use watch::watch;
