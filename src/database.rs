use std::env;

use actix::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use failure::Error;

use crate::index::Index;
use crate::storage::Storage;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub index: Addr<Index>,
    pub storage: Addr<Storage>,
    pub pool: Pool,
}

impl Actor for Database {
    type Context = SyncContext<Self>;
}

impl Database {
    pub fn connection(&self) -> Result<Connection, Error> {
        Ok(self.pool.get()?)
    }
}

pub fn connect() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("failed to connect to database");
    pool
}
