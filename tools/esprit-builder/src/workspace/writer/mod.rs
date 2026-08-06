use crate::{errors::Result, filesystem};

pub fn write_readme() -> Result<()> {
    filesystem::write("generated/README.md", crate::workspace::templates::README)?;

    Ok(())
}
