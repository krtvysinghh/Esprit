use camino::Utf8PathBuf;

#[derive(Clone, Debug)]
pub struct Project {
    pub root: Utf8PathBuf,
}

impl Project {
    pub fn discover() -> Self {
        Self {
            root: Utf8PathBuf::from("."),
        }
    }
}
