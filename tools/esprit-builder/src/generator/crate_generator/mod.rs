use camino::Utf8PathBuf;

use crate::{errors::Result, filesystem, generator::Generator};

pub struct CrateGenerator {
    pub name: String,
}

impl Generator for CrateGenerator {
    fn generate(&self) -> Result<()> {
        let root = Utf8PathBuf::from(format!("generated/{}", self.name));

        filesystem::create_dir(root.join("src"))?;

        filesystem::write(
            root.join("Cargo.toml"),
            &format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"
"#,
                self.name
            ),
        )?;

        filesystem::write(root.join("src/lib.rs"), "")?;

        Ok(())
    }
}
