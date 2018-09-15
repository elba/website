use actix_web::{http::Method, App};

use crate::{package, user, AppState};

pub fn router(app: App<AppState>) -> App<AppState> {
    app.prefix("/api/v1")
        .route("/users/login", Method::GET, user::controller::login)
        .route("/packages/search", Method::GET, package::controller::yank)
        .route(
            "/packages/groups",
            Method::GET,
            package::controller::metadata::list_groups,
        ).route(
            "/packages/{group}/metadata",
            Method::GET,
            package::controller::metadata::show_group,
        ).route(
            "/packages/{group}/packages",
            Method::GET,
            package::controller::metadata::list_packages,
        ).route(
            "/packages/{group}/{package}/metadata",
            Method::GET,
            package::controller::metadata::show_package,
        ).route(
            "/packages/{group}/{package}/versions",
            Method::GET,
            package::controller::metadata::list_versions,
        ).route(
            "/packages/{group}/{package}/{version}/metadata",
            Method::GET,
            package::controller::metadata::show_version,
        ).route(
            "/packages/{group}/{package}/{version}/readme",
            Method::GET,
            package::controller::metadata::show_readme,
        ).route(
            "/packages/{group}/{package}/{version}/dependencies",
            Method::GET,
            package::controller::download,
        ).route(
            "/packages/{group}/{package}/{version}/downloads",
            Method::GET,
            package::controller::download,
        ).route(
            "/packages/{group}/{package}/{version}/download",
            Method::GET,
            package::controller::download,
        ).route(
            "/packages/{group}/{package}/{version}/publish",
            Method::PUT,
            package::controller::publish,
        ).route(
            "/packages/{group}/{package}/{version}/yank",
            Method::PATCH,
            package::controller::yank,
        )
}
