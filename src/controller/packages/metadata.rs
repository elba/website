use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use futures::{future, prelude::*};

use crate::controller::users::UserMetadata;
use crate::model::packages::*;
use crate::util::error::report_error;
use crate::AppState;

use super::*;

pub fn list_groups(state: State<AppState>) -> impl Responder {
    let list_groups = state.db.send(ListGroups).from_err::<Error>().flatten();

    list_groups
        .map(|groups| {
            let groups = groups.into_iter().map(GroupView::from).collect();

            #[derive(Serialize)]
            struct R {
                groups: Vec<GroupView>,
            }

            HttpResponse::Ok().json(R { groups })
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
        .map(|packages| {
            let packages = packages.into_iter().map(PackageView::from).collect();

            #[derive(Serialize)]
            struct R {
                packages: Vec<PackageView>,
            }

            HttpResponse::Ok().json(R { packages })
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
        .map(|versions| {
            let versions = versions.into_iter().map(PackageVersionView::from).collect();

            #[derive(Serialize)]
            struct R {
                versions: Vec<PackageVersionView>,
            }

            HttpResponse::Ok().json(R { versions })
        }).or_else(report_error)
        .responder()
}

pub fn list_owners((path, state): (Path<PackageView>, State<AppState>)) -> impl Responder {
    let package_name = match PackageName::try_from(path.clone()) {
        Ok(package_name) => package_name,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_owners = state
        .db
        .send(ListOwners(package_name))
        .from_err::<Error>()
        .flatten();

    list_owners
        .map(|owners| {
            let owners = owners.into_iter().map(UserMetadata::from).collect();

            #[derive(Serialize)]
            struct R {
                owners: Vec<UserMetadata>,
            }

            HttpResponse::Ok().json(R { owners })
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
        .map(|dependencies| {
            let dependencies: Vec<_> = dependencies
                .into_iter()
                .map(DependencyMetadata::from)
                .collect();

            #[derive(Serialize)]
            struct R {
                dependencies: Vec<DependencyMetadata>,
            }

            HttpResponse::Ok().json(R { dependencies })
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

            #[derive(Serialize)]
            struct R {
                group: GroupMetadata,
            }

            Ok(HttpResponse::Ok().json(R { group: group_meta }))
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

            #[derive(Serialize)]
            struct R {
                package: PackageMetadata,
            }

            Ok(HttpResponse::Ok().json(R {
                package: package_meta,
            }))
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

            #[derive(Serialize)]
            struct R {
                version: VersionMetadata,
            }

            Ok(HttpResponse::Ok().json(R {
                version: version_meta,
            }))
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
        .map(move |readme| {
            #[derive(Serialize)]
            struct R {
                readme: String,
            }

            Ok(HttpResponse::Ok().json(R { readme }))
        }).flatten()
        .or_else(report_error)
        .responder()
}
