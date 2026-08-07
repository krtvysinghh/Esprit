use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Diagnostics {
    pub warnings: usize,
    pub errors: usize,
}

impl Diagnostics {
    pub fn healthy(&self) -> bool {
        self.errors == 0
    }
}
