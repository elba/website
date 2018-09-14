use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use futures::{future, prelude::*};

use super::*;

use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

pub fn list_groups(state: State<AppState>) -> impl Responder {
    let list_groups = state.db.send(ListGroups).from_err::<Error>().flatten();

    list_groups
        .map(|mut groups| {
            let groups: Vec<_> = groups.drain(..).map(GroupReq::from).collect();
            HttpResponse::Ok().json(groups)
        }).or_else(report_error)
        .responder()
}

pub fn list_packages((path, state): (Path<GroupReq>, State<AppState>)) -> impl Responder {
    let package_group = match PackageGroupName::try_from(path.clone()) {
        Ok(package_group) => package_group,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_packages = state
        .db
        .send(ListPackages(package_group))
        .from_err::<Error>()
        .flatten();

    list_packages
        .map(|mut packages| {
            let packages: Vec<_> = packages.drain(..).map(PackageReq::from).collect();
            HttpResponse::Ok().json(packages)
        }).or_else(report_error)
        .responder()
}

pub fn list_versions((path, state): (Path<PackageReq>, State<AppState>)) -> impl Responder {
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
            let versions: Vec<_> = versions.drain(..).map(PackageVersionReq::from).collect();
            HttpResponse::Ok().json(versions)
        }).or_else(report_error)
        .responder()
}

pub fn show_group((path, state): (Path<GroupReq>, State<AppState>)) -> impl Responder {
    let package_group = match PackageGroupName::try_from(path.clone()) {
        Ok(package_group) => package_group,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let lookup_group = state
        .db
        .send(LookupGroup(package_group.clone()))
        .from_err::<Error>()
        .flatten();

    lookup_group
        .map(move |group| {
            let group = group
                .ok_or_else(|| human!("Package group `{}` not found", &package_group.group()))?;

            let group_meta = GroupMetadata {
                group: path.into_inner(),
                created_at: group.created_at,
            };

            Ok(HttpResponse::Ok().json(group_meta))
        }).flatten()
        .or_else(report_error)
        .responder()
}

pub fn show_package((path, state): (Path<PackageReq>, State<AppState>)) -> impl Responder {
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
        .map(move |package| {
            let package =
                package.ok_or_else(|| human!("Package `{}` not found", &package_name.name(),))?;

            let package_meta = PackageMetadata {
                package: path.into_inner(),
                updated_at: package.updated_at,
                created_at: package.created_at,
            };

            Ok(HttpResponse::Ok().json(package_meta))
        }).flatten()
        .or_else(report_error)
        .responder()
}

pub fn show_version((path, state): (Path<PackageVersionReq>, State<AppState>)) -> impl Responder {
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
        .map(move |version| {
            let version = version.ok_or_else(|| {
                human!(
                    "Package version `{} {}` not found",
                    &package_version.name,
                    &package_version.semver
                )
            })?;

            let version_meta = VersionMetadata {
                package_version: path.into_inner(),
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
