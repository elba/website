use std::convert::TryFrom;

use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::packages::*;
use crate::util::empty_response;
use crate::AppState;

use super::PackageVersionReq;

#[derive(Deserialize, Clone)]
pub struct YankReq {
    pub yanked: bool,
    pub token: String,
}

pub async fn yank(
    (path, query, state): (Path<PackageVersionReq>, Query<YankReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.into_inner())?;

    await!(state.db.send(YankVersion {
        package: package_version.clone(),
        yanked: query.yanked,
        token: query.token.clone(),
    }))??;

    Ok(empty_response())
}
