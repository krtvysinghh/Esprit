use anyhow::Result;

pub trait Job {
    fn run(&self) -> Result<()>;
}

pub fn execute<J: Job>(job: J) -> Result<()> {
    job.run()
}
