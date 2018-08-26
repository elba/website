use std::env;

use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Database(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Database {
    type Context = SyncContext<Self>;
}

pub fn connect() -> Database {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed to connect to database.");
    Database(pool)
}
