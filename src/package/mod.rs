pub mod controller;
pub mod model;

#[derive(Clone)]
pub struct PackageName {
    pub group: String,
    pub name: String,
    pub group_normalized: String,
    pub name_normalized: String,
}

impl PackageName {
    pub fn new(package_group_name: &str, package_name: &str) -> PackageName {
        PackageName {
            group: package_group_name.to_owned(),
            name: package_name.to_owned(),
            group_normalized: normalize(package_group_name),
            name_normalized: normalize(package_name),
        }
    }
}

fn normalize(name: &str) -> String {
    name.replace("_", "-")
}

#[derive(Clone)]
pub struct PackageVersion {
    pub name: PackageName,
    pub semver: String,
}
