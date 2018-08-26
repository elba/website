use std::fs::File;
use std::io::Write;

use actix::prelude::*;
use bytes::Bytes;
use failure::Error;

use crate::package::PackageVersion;
use crate::CONFIG;

pub struct FileStorage;

impl Actor for FileStorage {
    type Context = SyncContext<Self>;
}

pub struct SavePackage {
    pub package: PackageVersion,
    pub bytes: Bytes,
}

impl Message for SavePackage {
    type Result = Result<(), Error>;
}

impl Handler<SavePackage> for FileStorage {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: SavePackage, _: &mut Self::Context) -> Self::Result {
        let path = CONFIG.tarball_path.join(format!(
            "{}_{}_{}.tar",
            &msg.package.name.group, &msg.package.name.name, &msg.package.semver
        ));

        let mut file = File::create(path)?;
        file.write_all(&msg.bytes)?;

        Ok(())
    }
}
