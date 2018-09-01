use std::time::SystemTime;

use actix::prelude::*;
use bytes::Bytes;
use diesel::{
    self,
    pg::upsert::{excluded, on_constraint},
    prelude::*,
};
use elba::package::manifest::PackageInfo;
use failure::{Error, ResultExt};
use futures::Future;

use crate::database::{Connection, Database};
use crate::index::{Index, SaveAndUpdate};
use crate::schema::{dependencies, package_groups, packages, version_authors, versions};
use crate::user::model::{lookup_user, LookupUser};

#[derive(Clone)]
pub struct PackageName {
    pub group: String,
    pub name: String,
    pub group_normalized: String,
    pub name_normalized: String,
}

impl PackageName {
    pub fn new(package_group_name: &str, package_name: &str) -> PackageName {
        PackageName {
            group: package_group_name.to_owned(),
            name: package_name.to_owned(),
            group_normalized: normalize(package_group_name),
            name_normalized: normalize(package_name),
        }
    }
}

fn normalize(name: &str) -> String {
    name.replace("_", "-")
}

#[derive(Clone)]
pub struct PackageVersion {
    pub name: PackageName,
    pub semver: String,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct PackageGroup {
    id: i32,
    user_id: i32,
    package_group_name: String,
    package_group_name_origin: String,
    created_at: SystemTime,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Package {
    id: i32,
    package_group_id: i32,
    package_name: String,
    package_name_origin: String,
    description: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    readme_file: Option<String>,
    license: Option<String>,
    updated_at: SystemTime,
    created_at: SystemTime,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Version {
    id: i32,
    package_id: i32,
    semver: String,
    created_at: SystemTime,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Dependency {
    id: i32,
    version_id: i32,
    package_id: i32,
    version_req: String,
}

#[derive(Insertable)]
#[table_name = "package_groups"]
struct CreatePackageGroup<'a> {
    user_id: i32,
    package_group_name: &'a str,
    package_group_name_origin: &'a str,
}

#[derive(Insertable)]
#[table_name = "packages"]
struct CreatePackage<'a> {
    package_group_id: i32,
    package_name: &'a str,
    package_name_origin: &'a str,
    description: Option<&'a str>,
    homepage: Option<&'a str>,
    repository: Option<&'a str>,
    readme_file: Option<&'a str>,
    license: Option<&'a str>,
    updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "versions"]
struct CreateVersion<'a> {
    package_id: i32,
    semver: &'a str,
}

#[derive(Insertable)]
#[table_name = "dependencies"]
struct CreateDependency<'a> {
    version_id: i32,
    package_id: i32,
    version_req: &'a str,
}

#[derive(Insertable)]
#[table_name = "version_authors"]
struct CreateAuthor<'a> {
    version_id: i32,
    name: &'a str,
}

#[derive(Clone)]
pub struct DependencyReq {
    pub name: PackageName,
    pub version_req: String,
}

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

pub struct LookupVersion(pub PackageVersion);

impl Message for VerifyVersion {
    type Result = Result<(), Error>;
}

impl Message for PublishVersion {
    type Result = Result<(), Error>;
}

impl Message for LookupVersion {
    type Result = Result<Option<Version>, Error>;
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

impl Handler<LookupVersion> for Database {
    type Result = Result<Option<Version>, Error>;

    fn handle(&mut self, msg: LookupVersion, _: &mut Self::Context) -> Self::Result {
        lookup_version(msg, &self.connection()?)
    }
}

pub fn verify_version(msg: VerifyVersion, conn: &Connection) -> Result<(), Error> {
    use schema::package_groups::dsl::*;
    use schema::users::dsl::*;

    let user = lookup_user(
        LookupUser {
            token: msg.token.clone(),
        },
        conn,
    )?;

    if user.is_none() {
        return Err(human!("User not found to token `{}`", &msg.token));
    }

    let mut token_result = package_groups
        .inner_join(users)
        .select(token)
        .filter(package_group_name.eq(&msg.package.name.group_normalized))
        .load::<String>(conn)?;

    if let Some(token_exist) = token_result.pop() {
        if token_exist != msg.token {
            return Err(human!(
                "Package group `{}` has already been taken",
                &msg.package.name.group
            ));
        }
    }

    let version = lookup_version(LookupVersion(msg.package.clone()), conn)?;

    if version.is_some() {
        return Err(human!(
            "Package `{}/{} {}` already exists",
            &msg.package.name.group,
            &msg.package.name.name,
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
                .select(packages::columns::id)
                .filter(
                    package_groups::columns::package_group_name.eq(&dep_req.name.group_normalized),
                ).filter(packages::columns::package_name.eq(&dep_req.name.name_normalized))
                .get_result::<i32>(conn)
                .optional()?;

            if let Some(dep_id) = dep_id {
                deps_info.push((dep_id, dep_req.version_req.clone()));
            } else {
                return Err(human!(
                    "Dependency `{}/{}` not found in index",
                    &dep_req.name.group,
                    &dep_req.name.name,
                ));
            }
        }

        let user = lookup_user(
            LookupUser {
                token: msg.token.clone(),
            },
            conn,
        )?.expect("User should exist");

        diesel::insert_into(package_groups::table)
            .values(CreatePackageGroup {
                user_id: user.id,
                package_group_name: &msg.package.name.group_normalized,
                package_group_name_origin: &msg.package.name.group,
            }).on_conflict_do_nothing()
            .execute(conn)?;

        let package_group_id = package_groups::table
            .select(package_groups::columns::id)
            .filter(
                package_groups::columns::package_group_name.eq(&msg.package.name.group_normalized),
            ).get_result::<i32>(conn)?;

        let package_id = diesel::insert_into(packages::table)
            .values(CreatePackage {
                package_group_id,
                package_name: &msg.package.name.name_normalized,
                package_name_origin: &msg.package.name.name,
                description: msg.package_info.description.as_ref().map(|s| s.as_str()),
                homepage: msg.package_info.homepage.as_ref().map(|s| s.as_str()),
                repository: msg.package_info.repository.as_ref().map(|s| s.as_str()),
                readme_file: msg.readme_file.as_ref().map(|s| s.as_str()),
                license: msg.package_info.license.as_ref().map(|s| s.as_str()),
                updated_at: SystemTime::now(),
            }).on_conflict(on_constraint("unique_group_package"))
            .do_update()
            .set((
                packages::columns::description.eq(excluded(packages::columns::description)),
                packages::columns::homepage.eq(excluded(packages::columns::homepage)),
                packages::columns::repository.eq(excluded(packages::columns::repository)),
                packages::columns::readme_file.eq(excluded(packages::columns::readme_file)),
                packages::columns::license.eq(excluded(packages::columns::license)),
                packages::columns::updated_at.eq(excluded(packages::columns::updated_at)),
            )).returning(packages::columns::id)
            .get_result::<i32>(conn)?;

        let version_id = diesel::insert_into(versions::table)
            .values(CreateVersion {
                package_id,
                semver: &msg.package.semver,
            }).returning(versions::columns::id)
            .get_result::<i32>(conn)?;

        let create_deps: Vec<CreateDependency> = deps_info
            .iter()
            .map(|dep_info| CreateDependency {
                version_id,
                package_id: dep_info.0,
                version_req: dep_info.1.as_str(),
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

pub fn lookup_version(msg: LookupVersion, conn: &Connection) -> Result<Option<Version>, Error> {
    use schema::package_groups::dsl::*;
    use schema::packages::dsl::*;
    use schema::versions::dsl::*;
    let version = versions
        .inner_join(packages.inner_join(package_groups))
        .filter(package_group_name.eq(&msg.0.name.group_normalized))
        .filter(package_name.eq(&msg.0.name.name_normalized))
        .filter(semver.eq(&msg.0.semver))
        .select(versions::all_columns())
        .get_result::<Version>(conn)
        .optional()?;
    Ok(version)
}
