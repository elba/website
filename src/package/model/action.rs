use actix::prelude::*;
use bytes::Bytes;
use chrono::offset::Utc;
use diesel::{
    self,
    pg::upsert::{excluded, on_constraint},
    prelude::*,
};
use elba::package::{manifest::PackageInfo, Name as PackageName};
use failure::{Error, ResultExt};
use futures::Future;

use crate::database::{Connection, Database};
use crate::index::{Index, SaveAndUpdate, YankAndUpdate};
use crate::schema::{dependencies, package_groups, packages, readmes, version_authors, versions};
use crate::user::model::{lookup_user, LookupUser};

use super::schema::*;
use super::*;

pub struct VerifyVersion {
    pub package: PackageVersion,
    pub token: String,
}

pub struct PublishVersion {
    pub package: PackageVersion,
    pub package_info: PackageInfo,
    pub readme_file: Option<String>,
    pub dependencies: Vec<(DependencyReq)>,
    pub token: String,
    pub bytes: Bytes,
}

pub struct YankVersion {
    pub package: PackageVersion,
    pub yanked: bool,
    pub token: String,
}

pub struct ListGroups;
pub struct ListPackages(pub PackageGroupName);
pub struct ListVersions(pub PackageName);
pub struct ListDependencies(pub PackageVersion);

pub struct LookupGroup(pub PackageGroupName);
pub struct LookupPackage(pub PackageName);
pub struct LookupVersion(pub PackageVersion);
pub struct LookupReadme(pub PackageVersion);

pub struct IncreaseDownload(pub PackageVersion);

impl Message for VerifyVersion {
    type Result = Result<(), Error>;
}

impl Message for PublishVersion {
    type Result = Result<(), Error>;
}

impl Message for YankVersion {
    type Result = Result<(), Error>;
}

impl Message for ListGroups {
    type Result = Result<Vec<PackageGroupName>, Error>;
}

impl Message for ListPackages {
    type Result = Result<Vec<PackageName>, Error>;
}

impl Message for ListVersions {
    type Result = Result<Vec<PackageVersion>, Error>;
}

impl Message for ListDependencies {
    type Result = Result<Vec<DependencyReq>, Error>;
}

impl Message for LookupGroup {
    type Result = Result<(PackageGroupName, PackageGroup), Error>;
}

impl Message for LookupPackage {
    type Result = Result<(PackageName, Package), Error>;
}

impl Message for LookupVersion {
    type Result = Result<(PackageVersion, Version), Error>;
}

impl Message for LookupReadme {
    type Result = Result<String, Error>;
}

impl Message for IncreaseDownload {
    type Result = Result<(), Error>;
}

impl Handler<VerifyVersion> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: VerifyVersion, _: &mut Self::Context) -> Self::Result {
        verify_version(msg, &self.connection()?)
    }
}

impl Handler<PublishVersion> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: PublishVersion, _: &mut Self::Context) -> Self::Result {
        publish_version(msg, &self.connection()?, &self.index)
    }
}

impl Handler<YankVersion> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: YankVersion, _: &mut Self::Context) -> Self::Result {
        yank_version(msg, &self.connection()?, &self.index)
    }
}

impl Handler<ListGroups> for Database {
    type Result = Result<Vec<PackageGroupName>, Error>;

    fn handle(&mut self, msg: ListGroups, _: &mut Self::Context) -> Self::Result {
        list_groups(msg, &self.connection()?)
    }
}

impl Handler<ListPackages> for Database {
    type Result = Result<Vec<PackageName>, Error>;

    fn handle(&mut self, msg: ListPackages, _: &mut Self::Context) -> Self::Result {
        list_packages(msg, &self.connection()?)
    }
}

impl Handler<ListVersions> for Database {
    type Result = Result<Vec<PackageVersion>, Error>;

    fn handle(&mut self, msg: ListVersions, _: &mut Self::Context) -> Self::Result {
        list_versions(msg, &self.connection()?)
    }
}

impl Handler<ListDependencies> for Database {
    type Result = Result<Vec<DependencyReq>, Error>;

    fn handle(&mut self, msg: ListDependencies, _: &mut Self::Context) -> Self::Result {
        list_dependencies(msg, &self.connection()?)
    }
}

impl Handler<LookupGroup> for Database {
    type Result = Result<(PackageGroupName, PackageGroup), Error>;

    fn handle(&mut self, msg: LookupGroup, _: &mut Self::Context) -> Self::Result {
        lookup_group(msg, &self.connection()?)
    }
}

impl Handler<LookupPackage> for Database {
    type Result = Result<(PackageName, Package), Error>;

    fn handle(&mut self, msg: LookupPackage, _: &mut Self::Context) -> Self::Result {
        lookup_package(msg, &self.connection()?)
    }
}

impl Handler<LookupVersion> for Database {
    type Result = Result<(PackageVersion, Version), Error>;

    fn handle(&mut self, msg: LookupVersion, _: &mut Self::Context) -> Self::Result {
        lookup_version(msg, &self.connection()?)
    }
}

impl Handler<LookupReadme> for Database {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: LookupReadme, _: &mut Self::Context) -> Self::Result {
        lookup_readme(msg, &self.connection()?)
    }
}

impl Handler<IncreaseDownload> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: IncreaseDownload, _: &mut Self::Context) -> Self::Result {
        increase_download(msg, &self.connection()?)
    }
}

pub fn verify_version(msg: VerifyVersion, conn: &Connection) -> Result<(), Error> {
    use schema::package_groups::dsl::*;
    use schema::users::dsl::*;

    let _ = lookup_user(
        LookupUser {
            token: msg.token.clone(),
        },
        conn,
    )?;

    let token_result = package_groups
        .inner_join(users)
        .select(token)
        .filter(package_group_name.eq(msg.package.name.normalized_group()))
        .get_result::<String>(conn)
        .optional()?;

    if let Some(token_exist) = token_result {
        if token_exist != msg.token {
            return Err(human!(
                "Package group `{}` has already been taken",
                &msg.package.name.group()
            ));
        }
    }

    let version = lookup_version(LookupVersion(msg.package.clone()), conn);

    // TODO
    if version.is_ok() {
        return Err(human!(
            "Package `{} {}` already exists",
            &msg.package.name.as_str(),
            &msg.package.semver,
        ));
    }

    Ok(())
}

pub fn publish_version(
    msg: PublishVersion,
    conn: &Connection,
    index: &Addr<Index>,
) -> Result<(), Error> {
    conn.build_transaction().serializable().run(|| {
        verify_version(
            VerifyVersion {
                token: msg.token.clone(),
                package: msg.package.clone(),
            },
            conn,
        )?;

        let mut deps_info = Vec::new();
        for dep_req in msg.dependencies.iter() {
            let dep_id = packages::table
                .inner_join(package_groups::table)
                .filter(package_groups::package_group_name.eq(&dep_req.name.normalized_group()))
                .filter(packages::package_name.eq(&dep_req.name.normalized_name()))
                .select(packages::id)
                .get_result::<i32>(conn)
                .optional()?;

            if let Some(dep_id) = dep_id {
                deps_info.push((dep_id, dep_req.version_req.clone()));
            } else {
                return Err(human!(
                    "Dependency `{}` not found in index",
                    dep_req.name.as_str()
                ));
            }
        }

        let user = lookup_user(
            LookupUser {
                token: msg.token.clone(),
            },
            conn,
        )?;

        diesel::insert_into(package_groups::table)
            .values(CreatePackageGroup {
                user_id: user.id,
                package_group_name: &msg.package.name.normalized_group(),
                package_group_name_origin: &msg.package.name.group(),
            }).on_conflict_do_nothing()
            .execute(conn)?;

        let package_group_id = package_groups::table
            .select(package_groups::id)
            .filter(package_groups::package_group_name.eq(&msg.package.name.normalized_group()))
            .get_result::<i32>(conn)?;

        let package_id = diesel::insert_into(packages::table)
            .values(CreatePackage {
                package_group_id,
                package_name: &msg.package.name.normalized_name(),
                package_name_origin: &msg.package.name.name(),
                updated_at: Utc::now().naive_utc(),
            }).on_conflict(on_constraint("unique_group_package"))
            .do_update()
            .set((packages::updated_at.eq(excluded(packages::updated_at)),))
            .returning(packages::id)
            .get_result::<i32>(conn)?;

        let version_id = diesel::insert_into(versions::table)
            .values(CreateVersion {
                package_id,
                semver: &msg.package.semver.to_string(),
                description: msg.package_info.description.as_ref().map(|s| s.as_str()),
                homepage: msg.package_info.homepage.as_ref().map(|s| s.as_str()),
                repository: msg.package_info.repository.as_ref().map(|s| s.as_str()),
                license: msg.package_info.license.as_ref().map(|s| s.as_str()),
            }).returning(versions::id)
            .get_result::<i32>(conn)?;

        if let Some(readme) = msg.readme_file {
            diesel::insert_into(readmes::table)
                .values(CreateReadme {
                    version_id,
                    textfile: &readme,
                }).execute(conn)?;
        }

        let create_deps: Vec<CreateDependency> = deps_info
            .iter()
            .map(|dep_info| CreateDependency {
                version_id,
                package_id: dep_info.0,
                version_req: dep_info.1.to_string(),
            }).collect();

        diesel::insert_into(dependencies::table)
            .values(create_deps)
            .execute(conn)?;

        let create_authors: Vec<CreateAuthor> = msg
            .package_info
            .authors
            .iter()
            .map(|name| CreateAuthor { version_id, name })
            .collect();

        diesel::insert_into(version_authors::table)
            .values(create_authors)
            .execute(conn)?;

        index
            .send(SaveAndUpdate {
                package: msg.package,
                dependencies: msg.dependencies,
                bytes: msg.bytes,
            }).from_err::<Error>()
            .wait()?
            .context("Failed to update index")?;

        Ok(())
    })
}

pub fn yank_version(msg: YankVersion, conn: &Connection, index: &Addr<Index>) -> Result<(), Error> {
    conn.build_transaction().serializable().run(|| {
        let user = lookup_user(
            LookupUser {
                token: msg.token.clone(),
            },
            conn,
        )?;

        let (_, group) = lookup_group(
            LookupGroup(PackageGroupName::new(msg.package.name.group().to_owned())?),
            conn,
        )?;
        let (_, version) = lookup_version(LookupVersion(msg.package.clone()), conn)?;

        if group.user_id != user.id {
            return Err(human!(
                "You don't own package `{}`",
                msg.package.name.as_str()
            ));
        }

        if version.yanked && msg.yanked {
            return Err(human!(
                "Package `{}` has already been yanked",
                msg.package.name.as_str()
            ));
        } else if !version.yanked && !msg.yanked {
            return Err(human!(
                "Can not unyank package `{}`, it is not yanked",
                msg.package.name.as_str()
            ));
        }

        diesel::update(versions::table)
            .filter(versions::id.eq(version.id))
            .set(versions::yanked.eq(msg.yanked))
            .execute(conn)?;

        index
            .send(YankAndUpdate {
                package: msg.package,
                yanked: msg.yanked,
            }).from_err::<Error>()
            .wait()?
            .context("Failed to yank/unyank version")?;

        Ok(())
    })
}

pub fn list_groups(_: ListGroups, conn: &Connection) -> Result<Vec<PackageGroupName>, Error> {
    let mut group_names = {
        use schema::package_groups::dsl::*;

        package_groups
            .select(package_group_name)
            .load::<String>(conn)?
    };

    let group_names: Vec<_> = group_names
        .drain(..)
        .filter_map(|group_name| PackageGroupName::new(group_name).ok())
        .collect();

    Ok(group_names)
}

pub fn list_packages(msg: ListPackages, conn: &Connection) -> Result<Vec<PackageName>, Error> {
    use schema::packages::dsl::*;

    let (group_name, package_group) = lookup_group(
        LookupGroup(PackageGroupName::new(msg.0.group().to_owned())?),
        conn,
    )?;

    let mut packages_names = Package::belonging_to(&package_group)
        .select(package_name)
        .load::<String>(conn)?;

    let packages_names: Vec<_> = packages_names
        .drain(..)
        .filter_map(|packages_name| {
            PackageName::new(group_name.group().to_owned(), packages_name).ok()
        }).collect();

    Ok(packages_names)
}

pub fn list_versions(msg: ListVersions, conn: &Connection) -> Result<Vec<PackageVersion>, Error> {
    use schema::versions::dsl::*;

    let (package_name, package) = lookup_package(LookupPackage(msg.0), conn)?;

    let mut packages_versions = Version::belonging_to(&package)
        .select(semver)
        .load::<String>(conn)?;

    let packages_versions: Vec<_> = packages_versions
        .drain(..)
        .filter_map(|packages_version| {
            Some(PackageVersion {
                name: package_name.clone(),
                semver: packages_version.parse().ok()?,
            })
        }).collect();

    Ok(packages_versions)
}

pub fn list_dependencies(
    msg: ListDependencies,
    conn: &Connection,
) -> Result<Vec<DependencyReq>, Error> {
    use schema::dependencies::dsl::*;
    use schema::package_groups::dsl::*;
    use schema::packages::dsl::*;

    let (_, version) = lookup_version(LookupVersion(msg.0), conn)?;

    let mut result = Dependency::belonging_to(&version)
        .inner_join(packages.inner_join(package_groups))
        .select((
            package_group_name_origin,
            package_name_origin,
            dependencies::all_columns(),
        )).load::<((String, String, Dependency))>(conn)?;

    let package_dependencies: Vec<_> = result
        .drain(..)
        .filter_map(|(group_name, packages_name, dependency)| {
            Some(DependencyReq {
                name: PackageName::new(group_name, packages_name).ok()?,
                version_req: dependency.version_req.parse().ok()?,
            })
        }).collect();

    Ok(package_dependencies)
}

pub fn lookup_group(
    msg: LookupGroup,
    conn: &Connection,
) -> Result<(PackageGroupName, PackageGroup), Error> {
    use schema::package_groups::dsl::*;

    let package_group = package_groups
        .filter(package_group_name.eq(&msg.0.normalized_group()))
        .first::<PackageGroup>(conn)
        .optional()?
        .ok_or_else(|| human!("Package group `{}` not found", msg.0.group(),))?;

    Ok((
        PackageGroupName::new(package_group.package_group_name_origin.clone())?,
        package_group,
    ))
}

pub fn lookup_package(
    msg: LookupPackage,
    conn: &Connection,
) -> Result<(PackageName, Package), Error> {
    use schema::packages::dsl::*;

    // TODO:
    let (_, package_group) = lookup_group(
        LookupGroup(PackageGroupName::new(msg.0.group().to_owned())?),
        conn,
    )?;

    let package = Package::belonging_to(&package_group)
        .filter(package_name.eq(&msg.0.normalized_name()))
        .first::<Package>(conn)
        .optional()?
        .ok_or_else(|| human!("Package `{}` not found", msg.0.as_str(),))?;

    Ok((
        PackageName::new(
            package_group.package_group_name_origin,
            package.package_name_origin.clone(),
        )?,
        package,
    ))
}

pub fn lookup_version(
    msg: LookupVersion,
    conn: &Connection,
) -> Result<(PackageVersion, Version), Error> {
    use schema::versions::dsl::*;

    // TODO:
    let (package_name, package) = lookup_package(LookupPackage(msg.0.name.clone()), conn)?;

    let version = Version::belonging_to(&package)
        .filter(semver.eq(msg.0.semver.to_string()))
        .first::<Version>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                "Package version `{} {}` not found",
                msg.0.name.as_str(),
                msg.0.semver
            )
        })?;

    Ok((
        PackageVersion {
            name: package_name,
            semver: version.semver.parse()?,
        },
        version,
    ))
}

pub fn lookup_readme(msg: LookupReadme, conn: &Connection) -> Result<String, Error> {
    let (_, version) = lookup_version(LookupVersion(msg.0.clone()), conn)?;

    let readme = Readme::belonging_to(&version)
        .first::<Readme>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                "Package readme for `{} {}` not found",
                msg.0.name.as_str(),
                msg.0.semver
            )
        })?;

    Ok(readme.textfile)
}

pub fn increase_download(msg: IncreaseDownload, conn: &Connection) -> Result<(), Error> {
    use schema::version_downloads::dsl::*;

    let (_, version) = lookup_version(LookupVersion(msg.0), conn)?;

    diesel::insert_into(version_downloads)
        .values(CreateVersionDownload {
            version_id: version.id,
        }).on_conflict(on_constraint("unique_version_date"))
        .do_update()
        .set(downloads.eq(downloads + 1))
        .execute(conn)?;

    Ok(())
}
