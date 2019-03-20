use std::convert::TryFrom;

use actix_web::*;
use elba::package::Name as PackageName;
use failure::Error;
use tokio_async_await::await;

use crate::controller::users::UserView;
use crate::model::packages::*;
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

pub async fn list_owners(
    (path, state): (Path<PackageReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_name = PackageName::try_from(path.clone())?;
    let owners = await!(state.db.send(ListOwners(package_name)))??;
    let owners = owners.into_iter().map(UserView::from).collect();

    #[derive(Serialize)]
    struct R {
        owners: Vec<UserView>,
    }

    Ok(HttpResponse::Ok().json(R { owners }))
}

pub async fn list_dependencies(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.clone())?;
    let dependencies = await!(state.db.send(ListDependencies(package_version)))??;
    let dependencies: Vec<_> = dependencies.into_iter().map(DependencyView::from).collect();

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

    let package_meta = PackageView {
        package: package_name.into(),
        latest_version: versions.pop().unwrap().into(),
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

    let version_meta = VersionView {
        package_version: package_version.into(),
        yanked: version.yanked,
        description: version.description,
        homepage: version.homepage,
        repository: version.repository,
        license: version.license,
        keywords: keywords,
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

pub async fn show_readme(
    (path, state): (Path<PackageVersionReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let package_version = PackageVersion::try_from(path.clone())?;
    let readme = await!(state.db.send(LookupReadme(package_version.clone())))??;

    #[derive(Serialize)]
    struct R {
        readme: String,
    }

    Ok(HttpResponse::Ok().json(R { readme }))
}
