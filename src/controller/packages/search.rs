use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::search;

use super::PackageReq;

#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub q: String,
}

pub async fn search(query: Query<SearchReq>) -> Result<HttpResponse, Error> {
    let packages = await!(search::search_package(query.into_inner().q))?;
    let packages = packages.into_iter().map(PackageReq::from).collect();

    #[derive(Serialize)]
    struct R {
        packages: Vec<PackageReq>,
    }

    Ok(HttpResponse::Ok().json(R { packages }))
}
