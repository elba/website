use actix_web::*;
use failure::Error;
use futures::prelude::*;

use crate::model::packages::*;
use crate::util::error::report_error;
use crate::AppState;

use super::PackageReq;

#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub q: String,
}

pub fn search((query, state): (Query<SearchReq>, State<AppState>)) -> impl Responder {
    let search_package = state
        .db
        .send(SearchPackage(query.into_inner().q))
        .from_err::<Error>()
        .flatten();

    search_package
        .map(|packages| {
            let packages: Vec<_> = packages.into_iter().map(PackageReq::from).collect();
            HttpResponse::Ok().json(packages)
        }).or_else(report_error)
        .responder()
}
