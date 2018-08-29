use std::env;

use actix::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use failure::Error;

use crate::index::Index;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub index: Addr<Index>,
    pub pool: Pool,
}

impl Actor for Database {
    type Context = SyncContext<Self>;
}

impl Database {
    pub fn new(pool: Pool, index: Addr<Index>) -> Self {
        Database { pool, index }
    }

    pub fn connection(&self) -> Result<Connection, Error> {
        Ok(self.pool.get()?)
    }
}

pub fn connect() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed to connect to database.");
    pool
}
