mod action;
mod schema;

use elba::package::{version::Constraint, Name as PackageName};
use failure::Error;
use semver;

pub use self::action::*;
pub use self::schema::*;

#[derive(Clone)]
pub struct GroupName {
    group: String,
    normalized: String,
}

// TODO: Move into `elba`?
impl GroupName {
    pub fn new(group: String) -> Result<Self, Error> {
        let group_valid = group
            .chars()
            .all(|x| x.is_alphanumeric() || x == '_' || x == '-');
        if !group_valid {
            bail!("group can only contain letters, numbers, _, and -")
        }

        let normalized = group
            .to_ascii_lowercase()
            .drain(..)
            .map(|c| if c == '_' { '-' } else { c })
            .collect::<String>();
        if normalized.is_empty() {
            bail!("group cannot be empty")
        }

        Ok(GroupName { group, normalized })
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn normalized_group(&self) -> &str {
        &self.normalized
    }
}

pub fn group_of_package(name: &PackageName) -> GroupName {
    GroupName {
        group: name.group().into(),
        normalized: name.normalized_group().into(),
    }
}

#[derive(Clone)]
pub struct PackageVersion {
    pub name: PackageName,
    pub semver: semver::Version,
}

#[derive(Clone)]
pub struct DependencyReq {
    pub name: PackageName,
    pub version_req: Constraint,
}
