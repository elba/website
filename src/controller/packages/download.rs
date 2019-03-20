use std::convert::TryFrom;

use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::index::get_location;
use crate::model::packages::*;
use crate::AppState;

use super::PackageVersionReq;

pub async fn download(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.into_inner())?;

    await!(state.db.send(IncreaseDownload(package_version.clone())))??;

    Ok(HttpResponse::TemporaryRedirect()
        .header("Location", get_location(&package_version))
        .finish())
}
