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

use super::model::{GroupName, PackageVersion};
use crate::util::error::Reason;

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

#[derive(Serialize, Clone)]
pub struct GroupMetadata {
    #[serde(flatten)]
    pub group: GroupView,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct PackageMetadata {
    #[serde(flatten)]
    pub package: PackageView,
    #[serde(with = "crate::util::rfc3339")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct VersionMetadata {
    #[serde(flatten)]
    pub package_version: PackageVersionView,
    pub yanked: bool,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct DependencyMetadata {
    #[serde(flatten)]
    pub package: PackageView,
    pub version_req: String,
}

impl TryFrom<GroupView> for GroupName {
    type Error = Error;

    fn try_from(req: GroupView) -> Result<GroupName, Self::Error> {
        Ok(GroupName::new(req.group)
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid group name: {}", err))?)
    }
}

impl From<GroupName> for GroupView {
    fn from(group: GroupName) -> GroupView {
        GroupView {
            group: group.group().to_owned(),
        }
    }
}

impl TryFrom<PackageView> for PackageName {
    type Error = Error;

    fn try_from(req: PackageView) -> Result<PackageName, Self::Error> {
        let name = PackageName::new(req.group, req.package)
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid package name: {}", err))?;
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
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid package name: {}", err))?;
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
