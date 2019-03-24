use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub max_upload_size: usize,
    pub storage_path: PathBuf,
    pub backend_url: String,
    pub cdn_url: String,
    pub index_path: PathBuf,
    pub index_no_sync: bool,
    pub remote_index_url: String,
    pub remote_index_user: String,
    pub remote_index_pwd: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            max_upload_size: read_env("MAX_UPLOAD_SIZE")
                .parse()
                .expect("MAX_UPLOAD_SIZE is expected to be number."),
            storage_path: read_env_path("STORAGE_PATH"),
            backend_url: read_env("BACKEND_URL"),
            cdn_url: read_env("CDN_URL"),
            index_path: read_env_path("INDEX_PATH"),
            index_no_sync: read_env("INDEX_NO_SYNC")
                .parse()
                .expect("INDEX_NO_SYNC is expected to be a boolean!"),
            remote_index_url: read_env("REMOTE_INDEX_URL"),
            remote_index_user: read_env("REMOTE_INDEX_USER"),
            remote_index_pwd: read_env("REMOTE_INDEX_PWD"),
        }
    }
}

fn read_env(env_name: &str) -> String {
    env::var(env_name).expect(&format!("Environment variable `{}` not set.", env_name))
}

fn read_env_path(env_name: &str) -> PathBuf {
    let path = PathBuf::from(read_env(env_name));

    fs::create_dir_all(&path).expect(&format!("Can not create dir `{:?}`", &path));

    if path.is_absolute() {
        path
    } else {
        env::current_dir()
            .unwrap()
            .join(path)
            .canonicalize()
            .unwrap()
    }
}
