use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::packages::*;
use crate::AppState;

use super::PackageReq;

#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub q: String,
}

pub async fn search(
    (query, state): (Query<SearchReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let packages = await!(state.db.send(SearchPackage(query.into_inner().q)))??;
    let packages: Vec<_> = packages.into_iter().map(PackageReq::from).collect();

    Ok(HttpResponse::Ok().json(packages))
}
