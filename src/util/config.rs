use std::env;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub upload_limit: usize,
    pub tarball_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            upload_limit: env::var("UPLOAD_LIMIT")
                .expect("UPLOAD_LIMIT not set!")
                .parse()
                .expect("UPLOAD_LIMIT is not a number!"),
            tarball_path: env::var("TARBALL_PATH")
                .expect("TARBALL_PATH not set!")
                .into(),
        }
    }
}
