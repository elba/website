use chrono::NaiveDateTime;

use actix;
use actix::prelude::*;
use diesel::{self, prelude::*};
use failure::Error;

use chrono::Utc;

use crate::database::{Connection, Database};
use crate::schema::*;
use crate::util::error::Reason;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub gh_id: i32,
    pub gh_name: String,
    pub gh_access_token: String,
    pub gh_avatar: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct AccessToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct CreateUserOrLogin {
    pub email: Option<String>,
    pub gh_id: i32,
    pub gh_name: String,
    pub gh_access_token: String,
    pub gh_avatar: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "access_tokens"]
pub struct CreateAccessToken {
    pub user_id: i32,
}

pub struct RemoveAccessToken {
    pub user_id: i32,
    pub access_token_id: i32,
}

pub struct ListAccessTokens {
    pub user_id: i32,
}

pub struct LookupUser {
    pub id: i32,
}

pub struct LookupUserByToken {
    pub access_token: String,
}

impl Message for CreateUserOrLogin {
    type Result = Result<User, Error>;
}

impl Message for CreateAccessToken {
    type Result = Result<AccessToken, Error>;
}

impl Message for RemoveAccessToken {
    type Result = Result<(), Error>;
}

impl Message for ListAccessTokens {
    type Result = Result<Vec<AccessToken>, Error>;
}

impl Message for LookupUser {
    type Result = Result<User, Error>;
}

impl Message for LookupUserByToken {
    type Result = Result<User, Error>;
}

impl Handler<CreateUserOrLogin> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUserOrLogin, _: &mut Self::Context) -> Self::Result {
        create_user_or_login(msg, &self.connection()?)
    }
}

impl Handler<CreateAccessToken> for Database {
    type Result = Result<AccessToken, Error>;

    fn handle(&mut self, msg: CreateAccessToken, _: &mut Self::Context) -> Self::Result {
        create_access_token(msg, &self.connection()?)
    }
}

impl Handler<RemoveAccessToken> for Database {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: RemoveAccessToken, _: &mut Self::Context) -> Self::Result {
        remove_access_token(msg, &self.connection()?)
    }
}

impl Handler<ListAccessTokens> for Database {
    type Result = Result<Vec<AccessToken>, Error>;

    fn handle(&mut self, msg: ListAccessTokens, _: &mut Self::Context) -> Self::Result {
        list_access_tokens(msg, &self.connection()?)
    }
}

impl Handler<LookupUser> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: LookupUser, _: &mut Self::Context) -> Self::Result {
        lookup_user(msg, &self.connection()?)
    }
}

pub fn create_user_or_login(msg: CreateUserOrLogin, conn: &Connection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&msg)
        .on_conflict(gh_id)
        .do_nothing()
        .execute(conn)?;

    let user = users.filter(gh_id.eq(msg.gh_id)).get_result::<User>(conn)?;

    Ok(user)
}

pub fn create_access_token(
    msg: CreateAccessToken,
    conn: &Connection,
) -> Result<AccessToken, Error> {
    use crate::schema::access_tokens::dsl::*;
    let access_token = diesel::insert_into(access_tokens)
        .values(&msg)
        .get_result::<AccessToken>(conn)?;
    Ok(access_token)
}

pub fn remove_access_token(msg: RemoveAccessToken, conn: &Connection) -> Result<(), Error> {
    use crate::schema::access_tokens::dsl::*;

    let access_token = access_tokens
        .find(msg.access_token_id)
        .get_result::<AccessToken>(conn)
        .optional()?;

    let access_token = match access_token {
        Some(access_token) => access_token,
        None => return Err(human!(Reason::TokenNotFound, "Access token not found")),
    };

    if access_token.user_id != msg.user_id {
        return Err(human!(
            Reason::NoPermission,
            "You have not permission to remove this access token",
        ));
    }

    diesel::delete(&access_token).execute(conn)?;

    Ok(())
}

pub fn list_access_tokens(
    msg: ListAccessTokens,
    conn: &Connection,
) -> Result<Vec<AccessToken>, Error> {
    use crate::schema::access_tokens::dsl::*;
    let access_token = access_tokens
        .filter(user_id.eq(msg.user_id))
        .load::<AccessToken>(conn)?;
    Ok(access_token)
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
    use crate::schema::access_tokens::dsl::*;
    use crate::schema::users::dsl::*;

    let access_token = access_tokens
        .filter(token.eq(&msg.access_token))
        .get_result::<AccessToken>(conn)
        .optional()?
        .ok_or_else(|| {
            human!(
                Reason::UserNotFound,
                "User not found to access token `{}`",
                &msg.access_token
            )
        })?;

    diesel::update(&access_token)
        .set(last_used_at.eq(Utc::now().naive_utc()))
        .execute(conn)?;

    let user = users.find(access_token.user_id).get_result::<User>(conn)?;

    Ok(user)
}
