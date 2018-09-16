use actix_web::*;
use failure::Error;
use futures::{future, prelude::*};

use super::PackageVersionView;

use crate::index::get_location;
use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

use std::convert::TryFrom;

pub fn download((path, state): (Path<PackageVersionView>, State<AppState>)) -> impl Responder {
    let package_version = match PackageVersion::try_from(path.into_inner()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let increase_download = state
        .db
        .send(IncreaseDownload(package_version.clone()))
        .from_err::<Error>()
        .flatten();

    increase_download
        .map(move |()| {
            HttpResponse::TemporaryRedirect()
                .header("Location", get_location(&package_version))
                .finish()
        }).or_else(report_error)
        .responder()
}
