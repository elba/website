use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::search::SearchPackage;
use crate::AppState;

use super::PackageReq;

#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub q: String,
}

pub async fn search(
    (query, state): (Query<SearchReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let packages = await!(state.search.send(SearchPackage {
        query: query.into_inner().q
    }))??;
    let packages = packages.into_iter().map(PackageReq::from).collect();

    #[derive(Serialize)]
    struct R {
        packages: Vec<PackageReq>,
    }

    Ok(HttpResponse::Ok().json(R { packages }))
}
