#![feature(crate_in_paths)]
#![feature(nll)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate bytes;
extern crate dotenv;
extern crate elba;
extern crate env_logger;
extern crate futures;
extern crate git2;
extern crate num_cpus;
extern crate semver;
extern crate serde_json;
extern crate tar;
extern crate tokio;
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

#[macro_use]
mod util;
mod database;
mod index;
mod package;
mod router;
mod schema;
mod user;

use std::env;

use actix::prelude::*;
use actix_web::{middleware, server, App};

use crate::database::Database;
use crate::index::Index;
use crate::util::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<Database>,
}

fn main() {
    dotenv::dotenv().ok();
    // env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_LOG", "actix_web=debug,info,warn");
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = env::var("BIND_TO").expect("BIND_TO not set!");
    let sys = System::new("elba-backaned");

    let db_pool = database::connect();

    // We would want only one actor to operate the index repo
    let index = SyncArbiter::start(1, move || Index::new());

    let db = Database::new(db_pool, index);
    let db_actor = SyncArbiter::start(num_cpus::get() * 4, move || db.clone());

    let app_state = AppState { db: db_actor };

    server::new(move || {
        let app = App::with_state(app_state.clone()).middleware(middleware::Logger::default());
        router::router(app)
    }).bind(&address)
    .expect(&format!("Can't bind to {}", &address))
    .start();

    sys.run();
}
