use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
    pub port: u16,
}
impl Config {
    pub fn new(base_path: String, port: u16) -> Self {
        Config {
            path: PathBuf::from(base_path),
            port,
        }
    }
}
