pub mod manifest;
pub mod templates;
pub mod writer;

use crate::errors::Result;

pub trait WorkspaceTask {
    fn run(&self) -> Result<()>;
}
