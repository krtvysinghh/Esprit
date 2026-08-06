pub mod crate_generator;
pub mod feature;
pub mod plugin;
pub mod workspace;

use crate::errors::Result;

pub trait Generator {
    fn generate(&self) -> Result<()>;
}
