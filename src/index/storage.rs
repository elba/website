use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use bytes::Bytes;
use rusoto_core::request::HttpClient;
use rusoto_core::ByteStream;
use rusoto_credential::StaticProvider;
use rusoto_s3::{PutObjectRequest, S3Client, S3 as _};

use failure::Error;

use crate::model::packages::PackageVersion;
use crate::CONFIG;

#[derive(Debug, Clone)]
pub enum StorageConfig {
    Local {
        path: PathBuf,
        url: String,
    },
    S3 {
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
    },
}

pub enum Storage {
    Local,
    S3 { client: S3Client },
}

impl Storage {
    pub fn new() -> Self {
        match &CONFIG.storage_config {
            StorageConfig::Local { .. } => Storage::Local,
            StorageConfig::S3 {
                access_key,
                secret_key,
                region,
                ..
            } => {
                let credential =
                    StaticProvider::new_minimal(access_key.clone(), secret_key.clone());

                Storage::S3 {
                    client: S3Client::new_with(
                        HttpClient::new().expect("Can not setup http client for S3 client"),
                        credential,
                        region.parse().expect("S3 region provided is not valid"),
                    ),
                }
            }
        }
    }

    pub fn store_package(
        &mut self,
        package: &PackageVersion,
        tarball: Bytes,
        readme: Option<String>,
    ) -> Result<(), Error> {
        match self {
            Storage::Local => {
                let local_path = match &CONFIG.storage_config {
                    StorageConfig::Local { path, .. } => path,
                    _ => unreachable!(),
                };

                // save tarball
                let tar_path = local_path.join(tarball_path(package));
                info!("Local storage: saving tarball to `{:?}`", &tar_path);
                let mut file = File::create(&tar_path)?;
                file.write_all(&tarball)?;

                // save readme
                if let Some(readme) = readme {
                    let readme_path = local_path.join(readme_path(package));
                    let mut file = File::create(&readme_path)?;
                    file.write_all(readme.as_bytes())?;
                }

                Ok(())
            }
            Storage::S3 { client } => {
                let bucket = match &CONFIG.storage_config {
                    StorageConfig::S3 { bucket, .. } => bucket,
                    _ => unreachable!(),
                };

                // upload tarball
                let mut future = client.put_object(PutObjectRequest {
                    bucket: bucket.to_owned(),
                    key: tarball_path(package),
                    body: Some(ByteStream::from(tarball.to_vec())),
                    content_type: Some("application/x-tar".to_owned()),
                    ..Default::default()
                });
                future.set_timeout(Duration::from_secs(10));
                future.sync()?;

                // upload readme
                if let Some(readme) = readme {
                    let mut future = client.put_object(PutObjectRequest {
                        bucket: bucket.to_owned(),
                        key: readme_path(package),
                        body: Some(ByteStream::from(readme.into_bytes())),
                        content_type: Some("text/html".to_owned()),
                        ..Default::default()
                    });
                    future.set_timeout(Duration::from_secs(10));
                    future.sync()?;
                }

                Ok(())
            }
        }
    }
}

pub fn get_tarball_location(package: &PackageVersion) -> String {
    format!("{}/{}", get_base_url(), &tarball_path(package))
}

pub fn get_readme_location(package: &PackageVersion) -> String {
    format!("{}/{}", get_base_url(), &readme_path(package))
}

fn get_base_url() -> String {
    match &CONFIG.storage_config {
        StorageConfig::Local { url, .. } => url.clone(),
        StorageConfig::S3 { bucket, region, .. } => {
            format!("https://s3-{}.amazonaws.com/{}", region, bucket)
        }
    }
}

fn tarball_path(package: &PackageVersion) -> String {
    format!(
        "tarballs/{}_{}_{}.tar.gz",
        &package.name.normalized_group(),
        &package.name.normalized_name(),
        &package.semver
    )
}

fn readme_path(package: &PackageVersion) -> String {
    format!(
        "readmes/{}_{}_{}.md",
        &package.name.normalized_group(),
        &package.name.normalized_name(),
        &package.semver
    )
}
