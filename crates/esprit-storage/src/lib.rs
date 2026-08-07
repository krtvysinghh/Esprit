use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

pub fn open(name: &str) -> Result<Connection> {
    let d = ProjectDirs::from("dev", "esprit", "esprit").unwrap();
    fs::create_dir_all(d.data_dir())?;
    Ok(Connection::open(d.data_dir().join(name))?)
}
