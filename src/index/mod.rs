pub mod repo;

use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Write};

use actix::prelude::*;
use elba::remote::{resolution::DirectRes, TomlDep, TomlEntry};
use failure::{Error, ResultExt};
use itertools::Itertools;
use serde_json;

use crate::model::packages::{DependencyReq, PackageVersion};
use crate::CONFIG;

use self::repo::IndexRepo;

pub struct Index {
    pub repo: IndexRepo,
}

impl Index {
    pub fn new() -> Result<Self, Error> {
        Ok(Index {
            repo: IndexRepo::init()?,
        })
    }
}

impl Actor for Index {
    type Context = Context<Self>;
}

pub struct UpdatePackage {
    pub package: PackageVersion,
    pub dependencies: Vec<(DependencyReq)>,
}

pub struct YankPackage {
    pub package: PackageVersion,
    pub yanked: bool,
}

impl Message for UpdatePackage {
    type Result = Result<(), Error>;
}

impl Message for YankPackage {
    type Result = Result<(), Error>;
}

impl Handler<UpdatePackage> for Index {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UpdatePackage, _: &mut Self::Context) -> Self::Result {
        info!(
            "Index: updating index for publishing `{} {}`",
            &msg.package.name.as_str(),
            &msg.package.semver
        );

        // git fetch
        self.repo.fetch_and_reset()?;

        let group_path = self.repo.index_dir.path().join(&msg.package.name.group());
        let meta_path = group_path.join(&msg.package.name.name());

        create_dir_all(&group_path)?;

        let content = if meta_path.exists() {
            let mut file = OpenOptions::new().read(true).open(&meta_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        } else {
            String::new()
        };

        // read entries
        let mut entries = parse_entries(&content);

        // fix potential violation
        entries
            .retain(|entry| entry.name != msg.package.name || entry.version != msg.package.semver);

        // insert metadata entry
        let mut metadata = TomlEntry::from(msg.package.clone());
        for dep in msg.dependencies {
            metadata.dependencies.push(TomlDep::from(dep));
        }
        entries.push(metadata);

        // save entries
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&meta_path)?;
        file.write_all(&serialize_entries(entries).as_bytes())?;
        file.sync_all()?;

        // git commit && git push
        self.repo
            .commit_and_push(
                &format!(
                    "Updating package `{}|{}`",
                    &msg.package.name.as_str(),
                    &msg.package.semver
                ),
                &meta_path,
            ).with_context(|_| "failed to push index to remote repo")?;

        Ok(())
    }
}

impl Handler<YankPackage> for Index {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: YankPackage, _: &mut Self::Context) -> Self::Result {
        info!(
            "Index: updating index for yanking `{} {}` (yanked={})",
            &msg.package.name.as_str(),
            &msg.package.semver,
            &msg.yanked,
        );

        // git fetch
        self.repo.fetch_and_reset()?;

        let group_path = self.repo.index_dir.path().join(&msg.package.name.group());
        let meta_path = group_path.join(&msg.package.name.name());

        if !meta_path.exists() {
            return Err(format_err!("metafile `{:?}` not found", &meta_path));
        }

        let mut file = OpenOptions::new().read(true).open(&meta_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // read entries
        let mut entries = parse_entries(&content);

        // modify metadata entry
        for entry in &mut entries {
            if entry.name == msg.package.name && entry.version == msg.package.semver {
                entry.yanked = msg.yanked;
            }
        }

        // save entries
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&meta_path)?;
        file.write_all(&serialize_entries(entries).as_bytes())?;
        file.sync_all()?;

        // git commit && git push
        self.repo
            .commit_and_push(
                &format!(
                    "Updating package `{}|{}`",
                    &msg.package.name.as_str(),
                    &msg.package.semver
                ),
                &meta_path,
            ).with_context(|_| "failed to push index to remote repo")?;

        Ok(())
    }
}

fn parse_entries(content: &str) -> Vec<TomlEntry> {
    content
        .split("\n")
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

fn serialize_entries(entries: Vec<TomlEntry>) -> String {
    entries
        .iter()
        .filter_map(|entry| serde_json::to_string(entry).ok())
        .join("\n")
}

impl From<PackageVersion> for TomlEntry {
    fn from(package: PackageVersion) -> Self {
        TomlEntry {
            name: package.name.clone(),
            version: package.semver.clone(),
            location: Some(DirectRes::Registry {
                registry: CONFIG.registry.clone(),
                name: package.name,
                version: package.semver,
            }),
            dependencies: Vec::new(),
            yanked: false,
        }
    }
}

impl From<DependencyReq> for TomlDep {
    fn from(req: DependencyReq) -> Self {
        TomlDep {
            name: req.name,
            index: None,
            req: req.version_req,
        }
    }
}
