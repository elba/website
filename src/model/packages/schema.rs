use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Identifiable, Queryable)]
pub struct Group {
    pub id: i32,
    pub group_name: String,
    pub group_name_origin: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset)]
#[belongs_to(Group)]
pub struct Package {
    pub id: i32,
    pub group_id: i32,
    pub package_name: String,
    pub package_name_origin: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Package)]
pub struct Version {
    pub id: i32,
    pub package_id: i32,
    pub semver: String,
    pub yanked: bool,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Version)]
#[table_name = "dependencies"]
pub struct Dependency {
    pub id: i32,
    pub version_id: i32,
    pub package_id: i32,
    pub version_req: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Package)]
pub struct PackageOwner {
    pub id: i32,
    pub package_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct CreateGroup<'a> {
    pub group_name: &'a str,
    pub group_name_origin: &'a str,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "packages"]
pub struct CreatePackage<'a> {
    pub group_id: i32,
    pub package_name: &'a str,
    pub package_name_origin: &'a str,
}

#[derive(Insertable)]
#[table_name = "versions"]
pub struct CreateVersion<'a> {
    pub package_id: i32,
    pub semver: &'a str,
    pub description: Option<&'a str>,
    pub homepage: Option<&'a str>,
    pub repository: Option<&'a str>,
    pub license: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "dependencies"]
pub struct CreateDependency {
    pub version_id: i32,
    pub package_id: i32,
    pub version_req: String,
}

#[derive(Insertable)]
#[table_name = "version_authors"]
pub struct CreateAuthor<'a> {
    pub version_id: i32,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "version_keywords"]
pub struct CreateKeyword<'a> {
    pub version_id: i32,
    pub keyword: &'a str,
}

#[derive(Insertable)]
#[table_name = "package_owners"]
pub struct CreateOwner {
    pub package_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "version_downloads"]
pub struct CreateVersionDownload {
    pub version_id: i32,
}
