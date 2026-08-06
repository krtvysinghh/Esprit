use camino::Utf8PathBuf;

pub fn cwd() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(std::env::current_dir().expect("cwd")).expect("utf8 path")
}
