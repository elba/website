#![feature(nll)]
#![feature(await_macro, futures_api, async_await)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
mod util;
mod controller;
mod database;
mod index;
mod model;
mod router;
mod schema;
mod search;

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
use crate::search::SearchEngine;
use crate::util::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<Database>,
    pub search_engine: Addr<SearchEngine>,
}

fn main() {
    dotenv::from_filename(".env.override").ok();
    dotenv::dotenv().ok();

    env_logger::init();

    let sys = System::new("elba-backaned");

    let db_pool = database::connect();

    // We would want only one actor to operate the index repo
    let index = SyncArbiter::start(1, move || Index::new());

    let search_engine = SyncArbiter::start(1, move || {
        SearchEngine::init().expect("Faild to initialize search engine")
    });

    let db = Database::new(db_pool, index, search_engine.clone());
    let db = SyncArbiter::start(num_cpus::get() * 4, move || db.clone());

    let app_state = AppState { db, search_engine };

    server::new(move || {
        let app = App::with_state(app_state.clone())
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("user_token")
                    .secure(false),
            ));
        router::router(app)
    }).bind(&CONFIG.bind_to)
    .expect(&format!("Can't bind to {}", &CONFIG.bind_to))
    .start();

    sys.run();
}
