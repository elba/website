use actix_web::*;
use elba::package::Name;
use failure::Error;
use futures::{future, prelude::*};
use semver;

use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

#[derive(Deserialize, Clone)]
pub struct YankReq {
    pub package_group_name: String,
    pub package_name: String,
    pub semver: semver::Version,
    pub yanked: bool,
    pub token: String,
}

pub fn yank((query, state): (Query<YankReq>, State<AppState>)) -> impl Responder {
    // TODO: These ugly codes should be fixed by async/await
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

    let yank_version = state
        .db
        .send(YankVersion {
            package: package_version.clone(),
            yanked: query.yanked,
            token: query.token.clone(),
        }).from_err::<Error>()
        .flatten();

    yank_version
        .map(|()| HttpResponse::Ok().finish())
        .or_else(report_error)
        .responder()
}
