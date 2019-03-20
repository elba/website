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

use crate::model::packages::*;
use crate::util::error::Reason;

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupReq {
    pub group: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PackageReq {
    pub group: String,
    pub package: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PackageVersionReq {
    pub group: String,
    pub package: String,
    pub version: String,
}

#[derive(Serialize, Clone)]
pub struct GroupView {
    #[serde(flatten)]
    pub group: GroupReq,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct PackageView {
    #[serde(flatten)]
    pub package: PackageReq,
    #[serde(with = "crate::util::rfc3339")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct VersionView {
    #[serde(flatten)]
    pub package_version: PackageVersionReq,
    pub yanked: bool,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub keywords: Vec<String>,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Clone)]
pub struct DependencyView {
    #[serde(flatten)]
    pub package: PackageReq,
    pub version_req: String,
}

impl TryFrom<GroupReq> for GroupName {
    type Error = Error;

    fn try_from(req: GroupReq) -> Result<GroupName, Self::Error> {
        Ok(GroupName::new(req.group)
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid group name: {}", err))?)
    }
}

impl From<GroupName> for GroupReq {
    fn from(group: GroupName) -> GroupReq {
        GroupReq {
            group: group.group().to_owned(),
        }
    }
}

impl TryFrom<PackageReq> for PackageName {
    type Error = Error;

    fn try_from(req: PackageReq) -> Result<PackageName, Self::Error> {
        let name = PackageName::new(req.group, req.package)
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid package name: {}", err))?;
        Ok(name)
    }
}

impl From<PackageName> for PackageReq {
    fn from(name: PackageName) -> PackageReq {
        PackageReq {
            group: name.group().to_owned(),
            package: name.name().to_owned(),
        }
    }
}

impl TryFrom<PackageVersionReq> for PackageVersion {
    type Error = Error;

    fn try_from(req: PackageVersionReq) -> Result<PackageVersion, Self::Error> {
        let name = PackageName::new(req.group, req.package)
            .with_context(|err| human!(Reason::InvalidFormat, "Invalid package name: {}", err))?;
        Ok(PackageVersion {
            name,
            semver: req.version.parse()?,
        })
    }
}

impl From<PackageVersion> for PackageVersionReq {
    fn from(package: PackageVersion) -> PackageVersionReq {
        PackageVersionReq {
            group: package.name.group().to_owned(),
            package: package.name.name().to_owned(),
            version: package.semver.to_string(),
        }
    }
}

impl From<DependencyReq> for DependencyView {
    fn from(dependency: DependencyReq) -> DependencyView {
        DependencyView {
            package: dependency.name.into(),
            version_req: dependency.version_req.to_string(),
        }
    }
}
