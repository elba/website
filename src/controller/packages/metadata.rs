use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use tokio_async_await::await;

use crate::controller::users::UserView;
use crate::model::packages::*;
use crate::storage;
use crate::AppState;

use super::*;

pub async fn list_groups(state: State<AppState>) -> Result<HttpResponse, Error> {
    let groups = await!(state.db.send(ListGroups))??;
    let groups = groups.into_iter().map(GroupReq::from).collect();

    #[derive(Serialize)]
    struct R {
        groups: Vec<GroupReq>,
    }

    Ok(HttpResponse::Ok().json(R { groups }))
}

pub async fn list_packages(
    (path, state): (Path<GroupReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let group = GroupName::try_from(path.clone())?;
    let packages = await!(state.db.send(ListPackages(group)))??;
    let packages = packages.into_iter().map(PackageReq::from).collect();

    #[derive(Serialize)]
    struct R {
        packages: Vec<PackageReq>,
    }

    Ok(HttpResponse::Ok().json(R { packages }))
}

pub async fn list_versions(
    (path, state): (Path<PackageReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_name = PackageName::try_from(path.clone())?;

    let mut versions = await!(state.db.send(ListVersions(package_name)))??;
    versions.sort_by(|lhs, rhs| rhs.semver.cmp(&lhs.semver));

    let versions = versions.into_iter().map(PackageVersionReq::from).collect();

    #[derive(Serialize)]
    struct R {
        versions: Vec<PackageVersionReq>,
    }

    Ok(HttpResponse::Ok().json(R { versions }))
}

pub async fn list_dependencies(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.clone())?;
    let dependencies = await!(state.db.send(ListDependencies(package_version)))??;
    let dependencies = dependencies.into_iter().map(DependencyView::from).collect();

    #[derive(Serialize)]
    struct R {
        dependencies: Vec<DependencyView>,
    }

    Ok(HttpResponse::Ok().json(R { dependencies }))
}

pub async fn show_group(
    (path, state): (Path<GroupReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let group_name = GroupName::try_from(path.clone())?;
    let (group_name, group) = await!(state.db.send(LookupGroup(group_name.clone())))??;
    let group_meta = GroupView {
        group: group_name.into(),
        created_at: group.created_at,
    };

    #[derive(Serialize)]
    struct R {
        group: GroupView,
    }

    Ok(HttpResponse::Ok().json(R { group: group_meta }))
}

pub async fn show_package(
    (path, state): (Path<PackageReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_name = PackageName::try_from(path.clone())?;
    let (package_name, package) = await!(state.db.send(LookupPackage(package_name.clone())))??;

    let mut versions = await!(state.db.send(ListVersions(package_name.clone())))??;
    versions.sort_by(|lhs, rhs| lhs.semver.cmp(&rhs.semver));

    let owners = await!(state.db.send(ListOwners(package_name.clone())))??;
    let owners: Vec<UserView> = owners.into_iter().map(UserView::from).collect();

    let latest_version: PackageVersion = versions
        .pop()
        .ok_or_else(|| format_err!("no version found for package {}", &package_name))?
        .into();

    let (package_version, latest_version) = await!(state.db.send(LookupVersion(latest_version)))??;

    let keywords = await!(state.db.send(ListKeywords {
        version_id: latest_version.id,
    }))??;

    let latest_version_meta = VersionView {
        package_version: package_version.into(),
        yanked: latest_version.yanked,
        description: latest_version.description,
        homepage: latest_version.homepage,
        repository: latest_version.repository,
        license: latest_version.license,
        keywords,
        owners,
        created_at: latest_version.created_at,
    };

    let package_meta = PackageView {
        latest_version: latest_version_meta,
        package: package_name.into(),
        updated_at: package.updated_at,
        created_at: package.created_at,
    };

    #[derive(Serialize)]
    struct R {
        package: PackageView,
    }

    Ok(HttpResponse::Ok().json(R {
        package: package_meta,
    }))
}

pub async fn show_version(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.clone())?;

    let (package_version, version) =
        await!(state.db.send(LookupVersion(package_version.clone())))??;

    let keywords = await!(state.db.send(ListKeywords {
        version_id: version.id,
    }))??;

    let owners = await!(state.db.send(ListOwners(package_version.name.clone())))??;
    let owners = owners.into_iter().map(UserView::from).collect();

    let version_meta = VersionView {
        package_version: package_version.into(),
        yanked: version.yanked,
        description: version.description,
        homepage: version.homepage,
        repository: version.repository,
        license: version.license,
        keywords,
        owners,
        created_at: version.created_at,
    };

    #[derive(Serialize)]
    struct R {
        version: VersionView,
    }

    Ok(HttpResponse::Ok().json(R {
        version: version_meta,
    }))
}

pub async fn show_readme(path: Path<PackageVersionReq>) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.clone())?;

    Ok(HttpResponse::TemporaryRedirect()
        .header("Location", storage::get_readme_location(&package_version))
        .finish())
}
