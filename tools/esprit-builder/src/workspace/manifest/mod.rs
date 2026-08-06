use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub name: String,
    pub version: String,
}
