#![feature(nll)]
#![feature(await_macro, futures_api, async_await)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
mod util;
mod controller;
mod database;
mod index;
mod login;
mod model;
mod router;
mod schema;
mod search;
mod storage;

extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, server, App};

use crate::database::Database;
use crate::index::Index;
use crate::login::GhLogin;
use crate::model::packages::PopulateSearch;
use crate::search::Search;
use crate::storage::Storage;
use crate::util::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Clone)]
pub struct AppState {
    pub login: Addr<GhLogin>,
    pub search: Addr<Search>,
    pub db: Addr<Database>,
}

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let mut sys = System::new("elba-registry");

    let index = Index::new().expect("faild to initialize index").start();
    let storage = Storage::new().expect("faild to initialize storage").start();
    let search = Search::new().start();

    let db_pool = database::connect();

    let db = Database {
        index,
        storage,
        search: search.clone(),
        pool: db_pool,
    };
    let db = SyncArbiter::start(num_cpus::get() * 4, move || db.clone());

    let login = GhLogin { db: db.clone() }.start();

    let app_state = AppState { db, search, login };

    sys.block_on(app_state.db.send(PopulateSearch))
        .unwrap()
        .expect("faild to populate search engine");

    server::new(move || {
        let app = App::with_state(app_state.clone())
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("user_token")
                    .secure(false),
            ));
        router::router(app)
    })
    .bind(&CONFIG.bind_to)
    .expect(&format!("can't bind to {}", &CONFIG.bind_to))
    .start();

    sys.run();
}
