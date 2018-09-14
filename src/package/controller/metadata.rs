use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use futures::{future, prelude::*};

use super::{GroupReq, PackageReq, PackageVersionReq};

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
