use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use actix::prelude::*;
use bytes::Bytes;
use failure::{Error, ResultExt as _};
use rusoto_core::request::HttpClient;
use rusoto_core::{ByteStream, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client, S3 as _};

use crate::model::packages::PackageVersion;
use crate::CONFIG;

#[derive(Debug, Clone)]
pub enum StorageConfig {
    Local {
        // path to store packages locally
        path: PathBuf,
        // url to retrieve local pacakages from public network
        url: String,
    },
    S3 {
        bucket: String,
        access_key: String,
        secret_key: String,
        region: Region,
        base_url: String,
    },
}

pub enum Storage {
    Local,
    S3 { client: S3Client },
}

impl Actor for Storage {
    type Context = Context<Self>;
}

pub struct StorePackage {
    pub package: PackageVersion,
    pub bytes: Bytes,
    pub readme: Option<String>,
}

impl Message for StorePackage {
    type Result = Result<StorageTransaction, Error>;
}

pub struct DeleteObject {
    pub path: String,
}

impl Message for DeleteObject {
    type Result = Result<(), Error>;
}

impl Storage {
    pub fn new() -> Result<Self, Error> {
        let storage = match &CONFIG.storage_config {
            StorageConfig::Local { path, .. } => {
                fs::create_dir_all(path.join("tarballs"))?;
                fs::create_dir_all(path.join("readmes"))?;

                Storage::Local
            }
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
                        HttpClient::new().with_context(|_| "can not setup S3 http client")?,
                        credential,
                        region.clone(),
                    ),
                }
            }
        };

        Ok(storage)
    }

    fn store_object(&mut self, path: &str, bytes: Vec<u8>) -> Result<(), Error> {
        match self {
            Storage::Local => {
                let local_path = match &CONFIG.storage_config {
                    StorageConfig::Local { path, .. } => path,
                    _ => unreachable!(),
                };

                let local_path = local_path.join(path);
                info!("Local storage: saving object to `{:?}`", &local_path);
                let mut file = File::create(&local_path)?;
                file.write_all(&bytes)?;

                Ok(())
            }
            Storage::S3 { client } => {
                let bucket = match &CONFIG.storage_config {
                    StorageConfig::S3 { bucket, .. } => bucket,
                    _ => unreachable!(),
                };

                info!("S3 storage: saving object to `{:?}`", &path);
                let mut future = client.put_object(PutObjectRequest {
                    bucket: bucket.to_owned(),
                    key: path.to_owned(),
                    body: Some(ByteStream::from(bytes)),
                    ..Default::default()
                });
                future.set_timeout(Duration::from_secs(10));
                future.sync()?;

                Ok(())
            }
        }
    }

    fn delete_object(&mut self, path: String) -> Result<(), Error> {
        match self {
            Storage::Local => {
                let local_path = match &CONFIG.storage_config {
                    StorageConfig::Local { path, .. } => path,
                    _ => unreachable!(),
                };

                info!("Local storage: deleting object `{:?}`", &path);
                fs::remove_file(local_path.join(path))?;

                Ok(())
            }
            Storage::S3 { client } => {
                let bucket = match &CONFIG.storage_config {
                    StorageConfig::S3 { bucket, .. } => bucket,
                    _ => unreachable!(),
                };

                info!("S3 storage: deleting object `{:?}`", &path);
                let mut future = client.delete_object(DeleteObjectRequest {
                    bucket: bucket.to_owned(),
                    key: path,
                    ..Default::default()
                });
                future.set_timeout(Duration::from_secs(10));
                future.sync()?;

                Ok(())
            }
        }
    }
}

impl Handler<StorePackage> for Storage {
    type Result = Result<StorageTransaction, Error>;

    fn handle(&mut self, msg: StorePackage, ctx: &mut Self::Context) -> Self::Result {
        let mut transaction = StorageTransaction::new(ctx.address());

        // save tarball
        let tar_path = tarball_path(&msg.package);
        self.store_object(&tar_path, msg.bytes.to_vec())?;
        transaction.paths.push(tar_path);

        // save readme
        if let Some(readme) = msg.readme {
            let readme_path = readme_path(&msg.package);
            self.store_object(&readme_path, readme.as_bytes().to_vec())?;
            transaction.paths.push(readme_path);
        }

        Ok(transaction)
    }
}

impl Handler<DeleteObject> for Storage {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteObject, _: &mut Self::Context) -> Self::Result {
        self.delete_object(msg.path)
    }
}

pub struct StorageTransaction {
    storage: Addr<Storage>,
    paths: Vec<String>,
    commited: bool,
}

impl StorageTransaction {
    pub fn new(storage: Addr<Storage>) -> Self {
        StorageTransaction {
            storage,
            paths: Vec::new(),
            commited: false,
        }
    }

    pub fn commit(mut self) {
        self.commited = true;
    }
}

impl Drop for StorageTransaction {
    fn drop(&mut self) {
        if !self.commited {
            info!("Rollback storage: {:?}", &self.paths);
            for path in self.paths.iter().cloned() {
                self.storage.do_send(DeleteObject { path });
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
        StorageConfig::S3 { base_url, .. } => base_url.clone(),
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
