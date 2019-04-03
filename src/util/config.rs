use std::env;
use std::fs;
use std::path::PathBuf;

use crate::index::storage::StorageConfig;

#[derive(Clone)]
pub struct Config {
    pub bind_to: String,
    pub max_upload_size: usize,
    pub storage_config: StorageConfig,
    pub backend_url: String,
    pub search_engine_path: PathBuf,
    pub local_index_path: PathBuf,
    pub remote_index_url: String,
    pub remote_index_user: Option<String>,
    pub remote_index_pwd: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        let storage_config = match read_env("STORAGE_STRATEGY").to_uppercase().as_str() {
            "LOCAL" => StorageConfig::Local {
                path: read_env_path("STORAGE_LOCAL_PATH"),
                url: read_env("STORAGE_LOCAL_URL"),
            },
            "S3" => StorageConfig::S3 {
                bucket: read_env("STORAGE_S3_BUCKET"),
                access_key: read_env("STORAGE_S3_ACCESS_KEY"),
                secret_key: read_env("STORAGE_S3_SECRET_KEY"),
                region: read_env("STORAGE_S3_REGION"),
            },
            _ => panic!("`STORAGE_STRATEGY` only accepts `LOCAL` or `S3`."),
        };

        Config {
            bind_to: read_env("BIND_TO"),
            max_upload_size: read_env("MAX_UPLOAD_SIZE")
                .parse()
                .expect("MAX_UPLOAD_SIZE is expected to be number."),
            storage_config,
            backend_url: read_env("BACKEND_URL"),
            search_engine_path: read_env_path("SEARCH_ENGINE_PATH"),
            local_index_path: read_env_path("LOCAL_INDEX_PATH"),
            remote_index_url: read_env("REMOTE_INDEX_URL"),
            remote_index_user: env::var("REMOTE_INDEX_USER").ok(),
            remote_index_pwd: env::var("REMOTE_INDEX_PWD").ok(),
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
