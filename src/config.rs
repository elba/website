use std::env;
use std::fs;
use std::path::PathBuf;

use elba::remote::Registry;

use crate::login::GhOAuthConfig;
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
    pub cors_origin: Option<String>,
    pub gh_oauth_config: Option<GhOAuthConfig>,
}

impl Config {
    pub fn from_env() -> Config {
        let gh_oauth_config = if read_env("GH_OAUTH_ENABLED")
            .parse::<bool>()
            .expect("`GH_OAUTH_ENABLED` should be a boolean")
        {
            Some(GhOAuthConfig {
                client_id: read_env("GH_CLIENT_ID"),
                client_secret: read_env("GH_CLIENT_SECRET"),
                success_redirect_url: read_env("GH_OAUTH_SUCCESS_REDIRECT_URL"),
            })
        } else {
            None
        };

        let storage_config = match read_env("STORAGE_STRATEGY").to_uppercase().as_str() {
            "LOCAL" => StorageConfig::Local {
                path: read_env_path("STORAGE_LOCAL_PATH"),
                url: read_env("STORAGE_LOCAL_URL"),
            },
            "S3" => StorageConfig::S3 {
                bucket: read_env("STORAGE_S3_BUCKET"),
                access_key: read_env("STORAGE_S3_ACCESS_KEY"),
                secret_key: read_env("STORAGE_S3_SECRET_KEY"),
                base_url: read_env("STORAGE_S3_BASE_URL"),
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
            gh_oauth_config,
            storage_config,
            registry: Registry {
                url: read_env("REGISTRY_URL")
                    .parse()
                    .expect("REGISTRY_URL is not a valid url."),
            },
            remote_index_url: read_env("REMOTE_INDEX_URL"),
            remote_index_user: read_optional("REMOTE_INDEX_USER"),
            remote_index_pwd: read_optional("REMOTE_INDEX_PWD"),
            index_bot_name: read_env("INDEX_BOT_NAME"),
            index_bot_email: read_env("INDEX_BOT_EMAIL"),
            cors_origin: read_optional("CORS_ORIGIN"),
        }
    }
}

fn read_env(env_name: &str) -> String {
    env::var(env_name).expect(&format!("Environment variable `{}` not set.", env_name))
}

fn read_optional(env_name: &str) -> Option<String> {
    env::var(env_name).ok().filter(|var| !var.is_empty())
}

fn read_env_path(env_name: &str) -> PathBuf {
    let path = PathBuf::from(read_env(env_name));
    fs::create_dir_all(&path).expect(&format!("Can not create dir `{:?}`", &path));
    path
}
