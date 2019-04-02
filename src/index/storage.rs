use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use failure::Error;

use crate::model::packages::PackageVersion;
use crate::CONFIG;

#[derive(Debug, Clone)]
pub enum StorageConfig {
    Local {
        path: PathBuf,
    },
    S3 {
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
    },
}

pub struct Storage;

impl Storage {
    pub fn new() -> Self {
        Storage
    }

    pub fn store_archive(&mut self, package: &PackageVersion, bytes: &[u8]) -> Result<(), Error> {
        // TODO: implement s3
        let storage_path = match &CONFIG.storage_config {
            StorageConfig::Local { path } => path,
            _ => unimplemented!(),
        };

        // store tarball
        let tar_path = storage_path.join(tar_name(package));

        info!("Saving tarball to `{:?}`", &tar_path);

        let mut file = File::create(&tar_path)?;
        file.write_all(bytes)?;

        Ok(())
    }
}

pub fn get_location(package: &PackageVersion) -> String {
    format!("{}/{}", &CONFIG.cdn_url, &tar_name(package))
}

fn tar_name(package: &PackageVersion) -> String {
    format!(
        "{}_{}_{}.tar.gz",
        &package.name.normalized_group(),
        &package.name.normalized_name(),
        &package.semver
    )
}
