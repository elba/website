#[macro_use]
pub mod error;
pub mod async_await;
pub mod rfc3339;

mod config;

pub use self::config::Config;

use actix_web::HttpResponse;

pub fn empty_response() -> HttpResponse {
    HttpResponse::Ok().body("{}")
}
