use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use futures::{future, prelude::*};

use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

use super::*;

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

pub fn list_groups(state: State<AppState>) -> impl Responder {
    let list_groups = state.db.send(ListGroups).from_err::<Error>().flatten();

    list_groups
        .map(|mut groups| {
            let groups: Vec<_> = groups.drain(..).map(GroupView::from).collect();
            HttpResponse::Ok().json(groups)
        }).or_else(report_error)
        .responder()
}

pub fn list_packages((path, state): (Path<GroupView>, State<AppState>)) -> impl Responder {
    let group = match GroupName::try_from(path.clone()) {
        Ok(group) => group,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_packages = state
        .db
        .send(ListPackages(group))
        .from_err::<Error>()
        .flatten();

    list_packages
        .map(|mut packages| {
            let packages: Vec<_> = packages.drain(..).map(PackageView::from).collect();
            HttpResponse::Ok().json(packages)
        }).or_else(report_error)
        .responder()
}

pub fn list_versions((path, state): (Path<PackageView>, State<AppState>)) -> impl Responder {
    let package_name = match PackageName::try_from(path.clone()) {
        Ok(package_name) => package_name,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_versions = state
        .db
        .send(ListVersions(package_name))
        .from_err::<Error>()
        .flatten();

    list_versions
        .map(|mut versions| {
            let versions: Vec<_> = versions.drain(..).map(PackageVersionView::from).collect();
            HttpResponse::Ok().json(versions)
        }).or_else(report_error)
        .responder()
}

pub fn list_dependencies(
    (path, state): (Path<PackageVersionView>, State<AppState>),
) -> impl Responder {
    let package_version = match PackageVersion::try_from(path.clone()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_dependencies = state
        .db
        .send(ListDependencies(package_version))
        .from_err::<Error>()
        .flatten();

    list_dependencies
        .map(|mut dependencies| {
            let versions: Vec<_> = dependencies
                .drain(..)
                .map(|dependency| DependencyMetadata {
                    package: dependency.name.into(),
                    version_req: dependency.version_req.to_string(),
                }).collect();
            HttpResponse::Ok().json(versions)
        }).or_else(report_error)
        .responder()
}

pub fn show_group((path, state): (Path<GroupView>, State<AppState>)) -> impl Responder {
    let group = match GroupName::try_from(path.clone()) {
        Ok(group) => group,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let lookup_group = state
        .db
        .send(LookupGroup(group.clone()))
        .from_err::<Error>()
        .flatten();

    lookup_group
        .map(move |(group_name, group)| {
            let group_meta = GroupMetadata {
                group: group_name.into(),
                created_at: group.created_at,
            };

            Ok(HttpResponse::Ok().json(group_meta))
        }).flatten()
        .or_else(report_error)
        .responder()
}

pub fn show_package((path, state): (Path<PackageView>, State<AppState>)) -> impl Responder {
    let package_name = match PackageName::try_from(path.clone()) {
        Ok(package_name) => package_name,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let lookup_package = state
        .db
        .send(LookupPackage(package_name.clone()))
        .from_err::<Error>()
        .flatten();

    lookup_package
        .map(move |(package_name, package)| {
            let package_meta = PackageMetadata {
                package: package_name.into(),
                updated_at: package.updated_at,
                created_at: package.created_at,
            };

            Ok(HttpResponse::Ok().json(package_meta))
        }).flatten()
        .or_else(report_error)
        .responder()
}

pub fn show_version((path, state): (Path<PackageVersionView>, State<AppState>)) -> impl Responder {
    let package_version = match PackageVersion::try_from(path.clone()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let lookup_version = state
        .db
        .send(LookupVersion(package_version.clone()))
        .from_err::<Error>()
        .flatten();

    lookup_version
        .map(move |(package_version, version)| {
            let version_meta = VersionMetadata {
                package_version: package_version.into(),
                yanked: version.yanked,
                description: version.description,
                homepage: version.homepage,
                repository: version.repository,
                license: version.license,
                created_at: version.created_at,
            };

            Ok(HttpResponse::Ok().json(version_meta))
        }).flatten()
        .or_else(report_error)
        .responder()
}

pub fn show_readme((path, state): (Path<PackageVersionView>, State<AppState>)) -> impl Responder {
    let package_version = match PackageVersion::try_from(path.clone()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let lookup_readme = state
        .db
        .send(LookupReadme(package_version.clone()))
        .from_err::<Error>()
        .flatten();

    lookup_readme
        .map(move |readme| Ok(HttpResponse::Ok().body(readme)))
        .flatten()
        .or_else(report_error)
        .responder()
}
