use actix_web::*;
use elba::package::Name;
use failure::Error;
use futures::{future, prelude::*};
use semver;

use crate::index::get_location;
use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

#[derive(Deserialize, Clone)]
pub struct DownloadReq {
    pub package_group_name: String,
    pub package_name: String,
    pub semver: semver::Version,
}

pub fn download((query, state): (Query<DownloadReq>, State<AppState>)) -> impl Responder {
    let name = match Name::new(query.package_group_name.clone(), query.package_name.clone()) {
        Ok(name) => name,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let package_version = PackageVersion {
        name,
        semver: query.semver.clone(),
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
