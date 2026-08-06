use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub root: Utf8PathBuf,
}

impl Project {
    pub fn new(root: Utf8PathBuf) -> Self {
        Self { root }
    }
}
