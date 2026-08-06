pub mod render;

use crate::errors::Result;

pub trait Template {
    fn render(&self) -> Result<String>;
}
