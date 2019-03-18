use actix::prelude::*;
use bytes::Bytes;
use chrono::offset::Utc;
use diesel::{self, pg::upsert::on_constraint, prelude::*};
use elba::package::{manifest::PackageInfo, Name as PackageName};
use failure::{Error, ResultExt};
use futures::Future;

use crate::database::{Connection, Database};
use crate::index::{Index, SaveAndUpdate, YankAndUpdate};
use crate::model::users::{lookup_user_by_token, LookupUserByToken, User};
use crate::schema::*;
use crate::util::error::Reason;

use super::schema::*;
use super::*;

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
pub struct ListPackages(pub GroupName);
pub struct ListVersions(pub PackageName);
pub struct ListOwners(pub PackageName);
pub struct ListDependencies(pub PackageVersion);

pub struct LookupGroup(pub GroupName);
pub struct LookupPackage(pub PackageName);
pub struct LookupVersion(pub PackageVersion);
pub struct LookupReadme(pub PackageVersion);

pub struct SearchPackage(pub String);

pub struct IncreaseDownload(pub PackageVersion);

impl Message for PublishVersion {
    type Result = Result<(), Error>;
}

impl Message for YankVersion {
    type Result = Result<(), Error>;
}

impl Message for ListGroups {
    type Result = Result<Vec<GroupName>, Error>;
}

impl Message for ListPackages {
    type Result = Result<Vec<PackageName>, Error>;
}

impl Message for ListVersions {
    type Result = Result<Vec<PackageVersion>, Error>;
}

impl Message for ListOwners {
    type Result = Result<Vec<User>, Error>;
}

impl Message for ListDependencies {
    type Result = Result<Vec<DependencyReq>, Error>;
}

impl Message for LookupGroup {
    type Result = Result<(GroupName, Group), Error>;
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

impl Message for SearchPackage {
    type Result = Result<Vec<PackageName>, Error>;
}

impl Message for IncreaseDownload {
    type Result = Result<(), Error>;
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
    type Result = Result<Vec<GroupName>, Error>;

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

impl Handler<ListOwners> for Database {
    type Result = Result<Vec<User>, Error>;

    fn handle(&mut self, msg: ListOwners, _: &mut Self::Context) -> Self::Result {
        list_owners(msg, &self.connection()?)
    }
}

impl Handler<ListDependencies> for Database {
    type Result = Result<Vec<DependencyReq>, Error>;

    fn handle(&mut self, msg: ListDependencies, _: &mut Self::Context) -> Self::Result {
        list_dependencies(msg, &self.connection()?)
    }
}

impl Handler<LookupGroup> for Database {
    type Result = Result<(GroupName, Group), Error>;

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

impl Handler<SearchPackage> for Database {
    type Result = Result<Vec<PackageName>, Error>;

    fn handle(&mut self, msg: SearchPackage, _: &mut Self::Context) -> Self::Result {
        search_package(msg, &self.connection()?)
    }
}

impl Handler<IncreaseDownload> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: IncreaseDownload, _: &mut Self::Context) -> Self::Result {
        increase_download(msg, &self.connection()?)
    }
}

pub fn publish_version(
    msg: PublishVersion,
    conn: &Connection,
    index: &Addr<Index>,
) -> Result<(), Error> {
    conn.build_transaction().serializable().run(|| {
        let user = lookup_user_by_token(
            LookupUserByToken {
                token: msg.token.clone(),
            },
            conn,
        )?;

        let group = groups::table
            .filter(groups::columns::group_name.eq(&msg.package.name.normalized_group()))
            .first::<Group>(conn)
            .optional()?;

        let group = match group {
            Some(group) => group,
            None => diesel::insert_into(groups::table)
                .values(CreateGroup {
                    group_name: &msg.package.name.normalized_group(),
                    group_name_origin: &msg.package.name.group(),
                    user_id: user.id,
                }).get_result(conn)?,
        };

        let package = Package::belonging_to(&group)
            .filter(packages::columns::package_name.eq(&msg.package.name.normalized_name()))
            .first::<Package>(conn)
            .optional()?;

        let mut package = match package {
            Some(package) => {
                let package_owners =
                    PackageOwner::belonging_to(&package).load::<PackageOwner>(conn)?;

                if !package_owners.iter().any(|owner| owner.user_id == user.id) {
                    return Err(human!(
                        Reason::NoPermission,
                        "You have no access permission to package `{}`",
                        &msg.package.name.as_str()
                    ));
                }

                package
            }
            None => {
                if group.user_id != user.id {
                    return Err(human!(
                        Reason::NoPermission,
                        "You have no permission to create package under group `{}`",
                        &msg.package.name.group()
                    ));
                }

                let package = diesel::insert_into(packages::table)
                    .values(CreatePackage {
                        group_id: group.id,
                        package_name: &msg.package.name.normalized_name(),
                        package_name_origin: &msg.package.name.name(),
                    }).get_result::<Package>(conn)?;

                diesel::insert_into(package_owners::table)
                    .values(CreateOwner {
                        package_id: package.id,
                        user_id: user.id,
                    }).execute(conn)?;

                package
            }
        };

        package.updated_at = Utc::now().naive_utc();
        let connection: &PgConnection = &*conn;
        let package: Package = package.save_changes(connection)?;

        let version = Version::belonging_to(&package)
            .filter(versions::columns::semver.eq(msg.package.semver.to_string()))
            .first::<Version>(conn)
            .optional()?;

        if version.is_some() {
            return Err(human!(
                Reason::NoPermission,
                "Package `{} {}` already exists",
                &msg.package.name.as_str(),
                &msg.package.semver,
            ));
        }

        let version = diesel::insert_into(versions::table)
            .values(CreateVersion {
                package_id: package.id,
                semver: &msg.package.semver.to_string(),
                description: msg.package_info.description.as_ref().map(|s| s.as_str()),
                homepage: msg.package_info.homepage.as_ref().map(|s| s.as_str()),
                repository: msg.package_info.repository.as_ref().map(|s| s.as_str()),
                license: msg.package_info.license.as_ref().map(|s| s.as_str()),
            }).get_result::<Version>(conn)?;

        let mut deps_info = Vec::new();
        for dep_req in msg.dependencies.iter() {
            let dep_id = packages::table
                .inner_join(groups::table)
                .filter(groups::group_name.eq(&dep_req.name.normalized_group()))
                .filter(packages::package_name.eq(&dep_req.name.normalized_name()))
                .select(packages::id)
                .get_result::<i32>(conn)
                .optional()?;

            if let Some(dep_id) = dep_id {
                deps_info.push((dep_id, dep_req.version_req.clone()));
            } else {
                return Err(human!(
                    Reason::DependencyNotFound,
                    "Dependency `{}` not found in index",
                    dep_req.name.as_str()
                ));
            }
        }

        let create_deps: Vec<CreateDependency> = deps_info
            .iter()
            .map(|dep_info| CreateDependency {
                version_id: version.id,
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
            .map(|name| CreateAuthor {
                version_id: version.id,
                name,
            }).collect();

        diesel::insert_into(version_authors::table)
            .values(create_authors)
            .execute(conn)?;

        if let Some(readme) = msg.readme_file {
            diesel::insert_into(readmes::table)
                .values(CreateReadme {
                    version_id: version.id,
                    textfile: &readme,
                }).execute(conn)?;
        }

        diesel::sql_query("REFRESH MATERIALIZED VIEW ts_vectors;").execute(conn)?;

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
        let user = lookup_user_by_token(
            LookupUserByToken {
                token: msg.token.clone(),
            },
            conn,
        )?;

        let (_, package) = lookup_package(LookupPackage(msg.package.name.clone()), conn)?;
        let (_, version) = lookup_version(LookupVersion(msg.package.clone()), conn)?;

        let package_owners = PackageOwner::belonging_to(&package).load::<PackageOwner>(conn)?;

        if !package_owners.iter().any(|owner| owner.user_id == user.id) {
            return Err(human!(
                Reason::NoPermission,
                "You have no access permission to package `{}`",
                &msg.package.name.as_str()
            ));
        }

        if version.yanked && msg.yanked {
            return Err(human!(
                Reason::NoPermission,
                "Package `{}` has already been yanked",
                msg.package.name.as_str()
            ));
        } else if !version.yanked && !msg.yanked {
            return Err(human!(
                Reason::NoPermission,
                "Can not unyank package `{}`, because it is not yanked",
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

pub fn list_groups(_: ListGroups, conn: &Connection) -> Result<Vec<GroupName>, Error> {
    let mut group_names = {
        use crate::schema::groups::dsl::*;

        groups.select(group_name).load::<String>(conn)?
    };

    let group_names: Vec<_> = group_names
        .drain(..)
        .filter_map(|group_name| GroupName::new(group_name).ok())
        .collect();

    Ok(group_names)
}

pub fn list_packages(msg: ListPackages, conn: &Connection) -> Result<Vec<PackageName>, Error> {
    use crate::schema::packages::dsl::*;

    let (group_name, group) =
        lookup_group(LookupGroup(GroupName::new(msg.0.group().to_owned())?), conn)?;

    let mut package_names = Package::belonging_to(&group)
        .select(package_name)
        .load::<String>(conn)?;

    let package_names: Vec<_> = package_names
        .drain(..)
        .filter_map(|packages_name| {
            PackageName::new(group_name.group().to_owned(), packages_name).ok()
        }).collect();

    Ok(package_names)
}

pub fn list_versions(msg: ListVersions, conn: &Connection) -> Result<Vec<PackageVersion>, Error> {
    use crate::schema::versions::dsl::*;

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

pub fn list_owners(msg: ListOwners, conn: &Connection) -> Result<Vec<User>, Error> {
    use crate::schema::users::dsl::*;

    let (_, package) = lookup_package(LookupPackage(msg.0), conn)?;

    let packages_owners = PackageOwner::belonging_to(&package)
        .inner_join(users)
        .select(users::all_columns())
        .load::<User>(conn)?;

    Ok(packages_owners)
}

pub fn list_dependencies(
    msg: ListDependencies,
    conn: &Connection,
) -> Result<Vec<DependencyReq>, Error> {
    use crate::schema::dependencies::dsl::*;
    use crate::schema::groups::dsl::*;
    use crate::schema::packages::dsl::*;

    let (_, version) = lookup_version(LookupVersion(msg.0), conn)?;

    let mut result = Dependency::belonging_to(&version)
        .inner_join(packages.inner_join(groups))
        .select((
            group_name_origin,
            package_name_origin,
            dependencies::all_columns(),
        )).load::<((String, String, Dependency))>(conn)?;

    let package_dependencies: Vec<_> = result
        .drain(..)
        .filter_map(|(groups_name, packages_name, dependency)| {
            Some(DependencyReq {
                name: PackageName::new(groups_name, packages_name).ok()?,
                version_req: dependency.version_req.parse().ok()?,
            })
        }).collect();

    Ok(package_dependencies)
}

pub fn lookup_group(msg: LookupGroup, conn: &Connection) -> Result<(GroupName, Group), Error> {
    use crate::schema::groups::dsl::*;

    let group = groups
        .filter(group_name.eq(&msg.0.normalized_group()))
        .first::<Group>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                Reason::PackageNotFound,
                "Package group `{}` not found",
                msg.0.group(),
            )
        })?;

    Ok((GroupName::new(group.group_name_origin.clone())?, group))
}

pub fn lookup_package(
    msg: LookupPackage,
    conn: &Connection,
) -> Result<(PackageName, Package), Error> {
    use crate::schema::packages::dsl::*;

    let (_, group) = lookup_group(LookupGroup(group_of_package(&msg.0)), conn)?;

    let package = Package::belonging_to(&group)
        .filter(package_name.eq(&msg.0.normalized_name()))
        .first::<Package>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                Reason::PackageNotFound,
                "Package `{}` not found",
                msg.0.as_str(),
            )
        })?;

    Ok((
        PackageName::new(group.group_name_origin, package.package_name_origin.clone())?,
        package,
    ))
}

pub fn lookup_version(
    msg: LookupVersion,
    conn: &Connection,
) -> Result<(PackageVersion, Version), Error> {
    use crate::schema::versions::dsl::*;

    // TODO:
    let (package_name, package) = lookup_package(LookupPackage(msg.0.name.clone()), conn)?;

    let version = Version::belonging_to(&package)
        .filter(semver.eq(msg.0.semver.to_string()))
        .first::<Version>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                Reason::PackageNotFound,
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
                Reason::PackageNotFound,
                "Package readme for `{} {}` not found",
                msg.0.name.as_str(),
                msg.0.semver
            )
        })?;

    Ok(readme.textfile)
}

pub fn increase_download(msg: IncreaseDownload, conn: &Connection) -> Result<(), Error> {
    use crate::schema::version_downloads::dsl::*;

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

pub fn search_package(msg: SearchPackage, conn: &Connection) -> Result<Vec<PackageName>, Error> {
    use crate::schema::ts_vectors::dsl::*;
    use diesel::dsl::*;
    use diesel_full_text_search::*;

    let tsquery = plainto_tsquery(msg.0);
    let rank = ts_rank_cd(document, tsquery.clone());

    let query = ts_vectors
        .select((group_name, package_name))
        .filter(tsquery.matches(document))
        .group_by((group_name, package_name))
        .order_by(max(rank).desc())
        .limit(50);

    let mut results = query.load::<(String, String)>(conn)?;
    let package_names: Vec<_> = results
        .drain(..)
        .filter_map(|(group, package)| PackageName::new(group, package).ok())
        .collect();

    Ok(package_names)
}
