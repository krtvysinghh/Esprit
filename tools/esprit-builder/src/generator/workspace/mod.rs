use crate::{errors::Result, generator::Generator, workspace::writer};

pub struct WorkspaceGenerator;

impl Generator for WorkspaceGenerator {
    fn generate(&self) -> Result<()> {
        writer::write_readme()?;
        Ok(())
    }
}
