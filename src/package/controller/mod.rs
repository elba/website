pub mod metadata;

mod download;
mod publish;
mod search;
mod yank;

pub use self::download::download;
pub use self::publish::publish;
pub use self::search::search;
pub use self::yank::yank;

use std::convert::TryFrom;

use chrono::NaiveDateTime;
use elba::package::Name as PackageName;
use failure::{Error, ResultExt};

use super::model::{PackageGroupName, PackageVersion};

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupView {
    pub group: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PackageView {
    pub group: String,
    pub package: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PackageVersionView {
    pub group: String,
    pub package: String,
    pub version: String,
}

impl TryFrom<GroupView> for PackageGroupName {
    type Error = Error;

    fn try_from(req: GroupView) -> Result<PackageGroupName, Self::Error> {
        Ok(PackageGroupName::new(req.group)?)
    }
}

impl From<PackageGroupName> for GroupView {
    fn from(group: PackageGroupName) -> GroupView {
        GroupView {
            group: group.group().to_owned(),
        }
    }
}

impl TryFrom<PackageView> for PackageName {
    type Error = Error;

    fn try_from(req: PackageView) -> Result<PackageName, Self::Error> {
        let name = PackageName::new(req.group, req.package)
            .with_context(|err| human!("Invalid package name: {}", err))?;
        Ok(name)
    }
}

impl From<PackageName> for PackageView {
    fn from(name: PackageName) -> PackageView {
        PackageView {
            group: name.group().to_owned(),
            package: name.name().to_owned(),
        }
    }
}

impl TryFrom<PackageVersionView> for PackageVersion {
    type Error = Error;

    fn try_from(req: PackageVersionView) -> Result<PackageVersion, Self::Error> {
        let name = PackageName::new(req.group, req.package)
            .with_context(|err| human!("Invalid package name: {}", err))?;
        Ok(PackageVersion {
            name,
            semver: req.version.parse()?,
        })
    }
}

impl From<PackageVersion> for PackageVersionView {
    fn from(package: PackageVersion) -> PackageVersionView {
        PackageVersionView {
            group: package.name.group().to_owned(),
            package: package.name.name().to_owned(),
            version: package.semver.to_string(),
        }
    }
}
