use crate::package::model::{DependencyReq, PackageVersion};
use crate::CONFIG;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<DependencyMeta>,
    pub yanked: bool,
    pub location: String,
}

#[derive(Serialize, Deserialize)]
pub struct DependencyMeta {
    pub name: String,
    pub req: String,
}

impl From<PackageVersion> for Metadata {
    fn from(package: PackageVersion) -> Self {
        Metadata {
            name: format!("{}/{}", &package.name.group, &package.name.name),
            location: format!(
                "tar+{}/{}_{}_{}.tar",
                &CONFIG.storage_url,
                &package.name.group_normalized,
                &package.name.name_normalized,
                &package.semver
            ),
            version: package.semver,
            dependencies: Vec::new(),
            yanked: false,
        }
    }
}

impl From<DependencyReq> for DependencyMeta {
    fn from(req: DependencyReq) -> Self {
        DependencyMeta {
            name: format!("{}/{}", &req.name.group, &req.name.name),
            req: req.version_req,
        }
    }
}
