use std::convert::TryFrom;

use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::packages::*;
use crate::storage;
use crate::AppState;

use super::*;

#[derive(Serialize, Clone)]
pub struct DownloadStatsView {
    pub total: u32,
    pub season: u32,
}

#[derive(Serialize, Clone)]
pub struct DownloadGraphView {
    #[serde(with = "crate::util::rfc3339")]
    pub date: NaiveDateTime,
    pub downloads: u32,
}

pub async fn download(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.into_inner())?;

    await!(state.db.send(IncreaseDownload(package_version.clone())))??;

    Ok(HttpResponse::TemporaryRedirect()
        .header("Location", storage::get_tarball_location(&package_version))
        .finish())
}

pub async fn download_stats(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.into_inner())?;

    let download_stats = await!(state.db.send(LookupDownloadStats(package_version.clone())))??;
    let download_stats = DownloadStatsView {
        season: download_stats.downloads_season as u32,
        total: download_stats.downloads_total as u32,
    };

    #[derive(Serialize)]
    struct R {
        download_stats: DownloadStatsView,
    }

    Ok(HttpResponse::Ok().json(R { download_stats }))
}

pub async fn download_graph(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.into_inner())?;

    let download_graph = await!(state.db.send(LookupDownloadGraph(package_version.clone())))??;
    let download_graph = download_graph
        .into_iter()
        .map(|graph| DownloadGraphView {
            date: graph.date.and_hms(0, 0, 0),
            downloads: graph.downloads as u32,
        })
        .collect();

    #[derive(Serialize)]
    struct R {
        download_graph: Vec<DownloadGraphView>,
    }

    Ok(HttpResponse::Ok().json(R { download_graph }))
}
