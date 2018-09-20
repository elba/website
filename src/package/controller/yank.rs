use std::convert::TryFrom;

use actix_web::*;
use failure::Error;
use futures::{future, prelude::*};

use crate::package::model::*;
use crate::util::error::report_error;
use crate::AppState;

use super::PackageVersionView;

#[derive(Deserialize, Clone)]
pub struct YankReq {
    #[serde(flatten)]
    pub package: PackageVersionView,
    pub yanked: bool,
    pub token: String,
}

pub fn yank((query, state): (Query<YankReq>, State<AppState>)) -> impl Responder {
    let package_version = match PackageVersion::try_from(query.package.clone()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
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
