use crate::package::model::{DependencyReq, PackageVersion};
use crate::CONFIG;

use elba::remote::{resolution::DirectRes, TomlDep, TomlEntry};

impl From<PackageVersion> for TomlEntry {
    fn from(package: PackageVersion) -> Self {
        TomlEntry {
            name: package.name.clone(),
            location: DirectRes::Tar {
                url: format!(
                    "{}/{}_{}_{}.tar",
                    &CONFIG.storage_url,
                    &package.name.normalized_group(),
                    &package.name.normalized_name(),
                    &package.semver
                ).parse()
                .unwrap(),
                cksum: None,
            },
            version: package.semver,
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
