use std::time::SystemTime;

use actix::prelude::*;
use diesel::{self, prelude::*};
use failure::Error;

use crate::schema::users;
use crate::util::Database;

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
    pub created_at: SystemTime,
    pub last_used_at: SystemTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct CreateUser {
    pub email: Option<String>,
    pub gh_id: i32,
    pub gh_name: String,
    pub gh_access_token: String,
    pub gh_avatar: Option<String>,
    pub last_used_at: SystemTime,
}

pub struct LookupUser {
    pub token: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Message for LookupUser {
    type Result = Result<Option<User>, Error>;
}

impl Handler<CreateUser> for Database {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;
        let user = diesel::insert_into(users)
            .values(&msg)
            .on_conflict(gh_id)
            .do_update()
            .set(&msg)
            .get_result::<User>(&self.0.get()?)?;
        Ok(user)
    }
}

impl Handler<LookupUser> for Database {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: LookupUser, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;
        let user = users
            .filter(token.eq(msg.token))
            .get_result::<User>(&self.0.get()?)
            .optional()?;
        Ok(user)
    }
}
