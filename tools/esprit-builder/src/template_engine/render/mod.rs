use crate::errors::Result;

pub fn render(template: &str) -> Result<String> {
    Ok(template.to_owned())
}
