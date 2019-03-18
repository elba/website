use chrono::NaiveDateTime;

use actix;
use actix::prelude::*;
use diesel::{self, prelude::*};
use failure::Error;

use crate::database::{Connection, Database};
use crate::schema::users;
use crate::util::error::Reason;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub gh_id: i32,
    pub gh_name: String,
    pub gh_access_token: String,
    pub gh_avatar: Option<String>,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct CreateUser {
    pub email: Option<String>,
    pub gh_id: i32,
    pub gh_name: String,
    pub gh_access_token: String,
    pub gh_avatar: Option<String>,
    pub last_used_at: NaiveDateTime,
}

pub struct LookupUser {
    pub id: i32,
}

pub struct LookupUserByToken {
    pub token: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Message for LookupUser {
    type Result = Result<User, Error>;
}

impl Message for LookupUserByToken {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        create_user(msg, &self.connection()?)
    }
}

impl Handler<LookupUser> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: LookupUser, _: &mut Self::Context) -> Self::Result {
        lookup_user(msg, &self.connection()?)
    }
}

impl Handler<LookupUserByToken> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: LookupUserByToken, _: &mut Self::Context) -> Self::Result {
        lookup_user_by_token(msg, &self.connection()?)
    }
}

pub fn create_user(msg: CreateUser, conn: &Connection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let user = diesel::insert_into(users)
        .values(&msg)
        .on_conflict(gh_id)
        .do_update()
        .set(&msg)
        .get_result::<User>(conn)?;
    Ok(user)
}

pub fn lookup_user(msg: LookupUser, conn: &Connection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let user = users
        .find(msg.id)
        .get_result::<User>(conn)
        .optional()?
        .ok_or_else(|| human!(Reason::UserNotFound, "User not found to id `{}`", &msg.id))?;
    Ok(user)
}

pub fn lookup_user_by_token(msg: LookupUserByToken, conn: &Connection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(token.eq(&msg.token))
        .get_result::<User>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                Reason::UserNotFound,
                "User not found to token `{}`",
                &msg.token
            )
        })?;
    Ok(user)
}
