#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
use anyhow::Result;

pub trait Job {
    fn run(&self) -> Result<()>;
}

pub fn execute<J: Job>(job: J) -> Result<()> {
    job.run()
}
