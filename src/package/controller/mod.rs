mod download;
mod publish;
mod search;
mod yank;

pub use self::download::download;
pub use self::publish::publish;
// pub use self::search::search;
pub use self::yank::yank;

use std::convert::TryFrom;

use elba::package::Name as PackageName;
use failure::Error;
use semver;

use super::model::PackageVersion;

#[derive(Deserialize, Clone)]
pub struct GroupReq {
    pub group: String,
}

#[derive(Deserialize, Clone)]
pub struct PackageReq {
    pub group: String,
    pub package: String,
}

#[derive(Deserialize, Clone)]
pub struct PackageVersionReq {
    pub group: String,
    pub package: String,
    pub version: semver::Version,
}

impl TryFrom<PackageReq> for PackageName {
    type Error = Error;

    fn try_from(req: PackageReq) -> Result<PackageName, Self::Error> {
        Ok(PackageName::new(req.group, req.package)?)
    }
}

impl TryFrom<PackageVersionReq> for PackageVersion {
    type Error = Error;

    fn try_from(req: PackageVersionReq) -> Result<PackageVersion, Self::Error> {
        let name = PackageName::new(req.group, req.package)?;
        Ok(PackageVersion {
            name,
            semver: req.version,
        })
    }
}
