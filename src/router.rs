use actix_web::{http::Method, App};

use crate::{package, user, AppState};

pub fn router(app: App<AppState>) -> App<AppState> {
    app.prefix("/api/v1")
        .route("/users/login", Method::GET, user::controller::login)
        .route(
            "/packages/publish",
            Method::POST,
            package::controller::publish,
        ).route("/packages/yank", Method::POST, package::controller::yank)
        // .route(
        //     "/packages/search",
        //     Method::GET,
        //     package::controller::search,
        // )
        .route(
            "/packages/download",
            Method::GET,
            package::controller::download,
        )
}
