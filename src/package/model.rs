use std::time::SystemTime;

use actix::prelude::*;
use diesel::{
    self,
    pg::upsert::{excluded, on_constraint},
    prelude::*,
};
use failure::Error;

use super::*;
use crate::schema::{dependencies, package_groups, packages, versions};
use crate::user::model::LookupUser;
use crate::util::Database;

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
    updated_at: SystemTime,
    created_at: SystemTime,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Version {
    id: i32,
    package_id: i32,
    semver: String,
    description: Option<String>,
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
struct CreatePacakgeGroup<'a> {
    user_id: i32,
    package_group_name: &'a str,
    package_group_name_origin: &'a str,
}

#[derive(Insertable)]
#[table_name = "packages"]
struct CreatePacakge<'a> {
    package_group_id: i32,
    package_name: &'a str,
    package_name_origin: &'a str,
    description: Option<&'a str>,
    updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "versions"]
struct CreateVersion<'a> {
    package_id: i32,
    semver: &'a str,
    description: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "dependencies"]
struct CreateDependency<'a> {
    version_id: i32,
    package_id: i32,
    version_req: &'a str,
}

#[derive(Clone)]
pub struct PublishSpec {
    pub package: PackageVersion,
    pub token: String,
}

pub struct PublishMeta {
    pub description: Option<String>,
    pub dependencies: Vec<(DependencyReq)>,
}

pub struct DependencyReq {
    pub name: PackageName,
    pub version_req: String,
}

pub struct Verify(pub PublishSpec);

pub struct Publish(pub PublishSpec, pub PublishMeta);

pub struct LookupVersion(pub PackageVersion);

impl Message for Verify {
    type Result = Result<(), Error>;
}

impl Message for Publish {
    type Result = Result<(), Error>;
}

impl Message for LookupVersion {
    type Result = Result<Option<Version>, Error>;
}

impl Handler<Verify> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: Verify, ctx: &mut Self::Context) -> Self::Result {
        use schema::package_groups::dsl::*;
        use schema::users::dsl::*;

        let spec = msg.0;

        let user = self.handle(
            LookupUser {
                token: spec.token.clone(),
            },
            ctx,
        )?;

        if user.is_none() {
            return Err(format_err!("User not found to token '{}'.", &spec.token));
        }

        let mut token_result = package_groups
            .inner_join(users)
            .select(token)
            .filter(package_group_name.eq(&spec.package.name.group_normalized))
            .load::<String>(&self.0.get()?)?;

        if let Some(token_exist) = token_result.pop() {
            if token_exist != spec.token {
                return Err(format_err!(
                    "Package group '{}' has been taken.",
                    &spec.package.name.group
                ));
            }
        }

        let version = self.handle(LookupVersion(spec.package.clone()), ctx)?;

        if version.is_some() {
            return Err(format_err!(
                "Package {}/{} {} already exists.",
                &spec.package.name.group,
                &spec.package.name.name,
                &spec.package.semver,
            ));
        }

        Ok(())
    }
}

impl Handler<Publish> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: Publish, ctx: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let Publish(spec, info) = msg;

        conn.build_transaction().serializable().run(|| {
            self.handle(Verify(spec.clone()), ctx)?;

            let mut deps_info = Vec::new();
            for dep_req in info.dependencies {
                let dep_id = packages::table
                    .inner_join(package_groups::table)
                    .select(packages::columns::id)
                    .filter(
                        package_groups::columns::package_group_name
                            .eq(&spec.package.name.group_normalized),
                    )
                    .filter(packages::columns::package_name.eq(&spec.package.name.name_normalized))
                    .get_result::<i32>(&conn)
                    .optional()?;

                if let Some(dep_id) = dep_id {
                    deps_info.push((dep_id, dep_req.version_req.clone()));
                } else {
                    return Err(format_err!(
                        "Dependency {}/{} not found in index.",
                        &spec.package.name.group,
                        &spec.package.name.name,
                    ));
                }
            }

            let description = info.description.as_ref().map(String::as_str);

            let user = self
                .handle(
                    LookupUser {
                        token: spec.token.clone(),
                    },
                    ctx,
                )?
                .unwrap();

            let package_group = diesel::insert_into(package_groups::table)
                .values(CreatePacakgeGroup {
                    user_id: user.id,
                    package_group_name: &spec.package.name.group_normalized,
                    package_group_name_origin: &spec.package.name.group,
                })
                .on_conflict(package_groups::columns::package_group_name)
                .do_nothing()
                .get_result::<PackageGroup>(&conn)?;

            let package = diesel::insert_into(packages::table)
                .values(CreatePacakge {
                    package_group_id: package_group.id,
                    package_name: &spec.package.name.name_normalized,
                    package_name_origin: &spec.package.name.name,
                    description,
                    updated_at: SystemTime::now(),
                })
                .on_conflict(on_constraint("unique_group_package"))
                .do_update()
                .set((
                    packages::columns::description.eq(excluded(packages::columns::description)),
                    packages::columns::updated_at.eq(excluded(packages::columns::updated_at)),
                ))
                .get_result::<Package>(&conn)?;

            let version = diesel::insert_into(versions::table)
                .values(CreateVersion {
                    package_id: package.id,
                    semver: &spec.package.semver,
                    description,
                })
                .get_result::<Version>(&conn)?;

            let create_deps: Vec<CreateDependency> = deps_info
                .iter()
                .map(|dep_info| CreateDependency {
                    version_id: version.id,
                    package_id: dep_info.0,
                    version_req: dep_info.1.as_str(),
                })
                .collect();

            diesel::insert_into(dependencies::table)
                .values(create_deps)
                .execute(&conn)?;

            Ok(())
        })
    }
}

impl Handler<LookupVersion> for Database {
    type Result = Result<Option<Version>, Error>;

    fn handle(&mut self, msg: LookupVersion, _: &mut Self::Context) -> Self::Result {
        use schema::package_groups::dsl::*;
        use schema::packages::dsl::*;
        use schema::versions::dsl::*;
        let version = versions
            .inner_join(packages.inner_join(package_groups))
            .filter(package_group_name.eq(&msg.0.name.group_normalized))
            .filter(package_name.eq(&msg.0.name.name_normalized))
            .filter(semver.eq(&msg.0.semver))
            .select(versions::all_columns())
            .get_result::<Version>(&self.0.get()?)
            .optional()?;
        Ok(version)
    }
}
