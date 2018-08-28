pub mod metadata;
pub mod repo;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;

use actix::prelude::*;
use bytes::Bytes;
use failure::Error;
use serde_json;

use self::metadata::{DependencyMeta, Metadata};
use self::repo::IndexRepo;
use crate::package::model::{DependencyReq, PackageVersion};
use crate::CONFIG;

pub struct Index {
    pub repo: IndexRepo,
}

impl Index {
    pub fn new() -> Self {
        Index {
            repo: IndexRepo::init().expect("Failed to init index repo."),
        }
    }
}

impl Actor for Index {
    type Context = SyncContext<Self>;
}

pub struct SaveAndUpdate {
    pub package: PackageVersion,
    pub dependencies: Vec<(DependencyReq)>,
    pub bytes: Bytes,
}

impl Message for SaveAndUpdate {
    type Result = Result<(), Error>;
}

impl Handler<SaveAndUpdate> for Index {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: SaveAndUpdate, _: &mut Self::Context) -> Self::Result {
        // store tarball
        let tar_path = CONFIG.storage_path.join(format!(
            "{}_{}_{}.tar",
            &msg.package.name.group_normalized,
            &msg.package.name.name_normalized,
            &msg.package.semver
        ));

        let mut file = File::create(tar_path)?;
        file.write_all(&msg.bytes)?;

        // create metadata file
        let mut metadata = Metadata::from(msg.package.clone());
        for dep in msg.dependencies {
            metadata.dependencies.push(DependencyMeta::from(dep));
        }

        let group_path = CONFIG.index_path.join(&msg.package.name.group);
        let meta_path = group_path.join(&msg.package.name.name);

        create_dir_all(&group_path)?;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&meta_path)?;
        let mut buf = serde_json::to_string(&metadata)?;
        buf.push('\n');
        file.write_all(&buf.as_bytes())?;

        // git push
        self.repo.commit_and_push(
            &format!(
                "Updating package `{}/{}#{}`",
                &msg.package.name.group, &msg.package.name.name, &msg.package.semver
            ),
            &meta_path,
        )?;

        Ok(())
    }
}
