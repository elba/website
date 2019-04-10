use std::env;
use std::fs;
use std::path::PathBuf;

use elba::remote::Registry;

use crate::storage::StorageConfig;

#[derive(Clone)]
pub struct Config {
    pub bind_to: String,
    pub max_upload_size: usize,
    pub storage_config: StorageConfig,
    pub registry: Registry,
    pub remote_index_url: String,
    pub remote_index_user: Option<String>,
    pub remote_index_pwd: Option<String>,
    pub index_bot_name: String,
    pub index_bot_email: String,
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
                region: read_env("STORAGE_S3_REGION")
                    .parse()
                    .expect("STORAGE_S3_REGION is not a valid AWS region"),
            },
            _ => panic!("`STORAGE_STRATEGY` only accepts `LOCAL` or `S3`."),
        };

        Config {
            bind_to: read_env("BIND_TO"),
            max_upload_size: read_env("MAX_UPLOAD_SIZE")
                .parse()
                .expect("MAX_UPLOAD_SIZE is expected to be number."),
            storage_config,
            registry: Registry {
                url: read_env("REGISTRY_URL")
                    .parse()
                    .expect("REGISTRY_URL is not a valid url."),
            },
            remote_index_url: read_env("REMOTE_INDEX_URL"),
            remote_index_user: env::var("REMOTE_INDEX_USER").ok(),
            remote_index_pwd: env::var("REMOTE_INDEX_PWD").ok(),
            index_bot_name: read_env("INDEX_BOT_NAME"),
            index_bot_email: read_env("INDEX_BOT_EMAIL"),
        }
    }
}

fn read_env(env_name: &str) -> String {
    env::var(env_name).expect(&format!("Environment variable `{}` not set.", env_name))
}

fn read_env_path(env_name: &str) -> PathBuf {
    let path = PathBuf::from(read_env(env_name));
    fs::create_dir_all(&path).expect(&format!("Can not create dir `{:?}`", &path));
    path
}
