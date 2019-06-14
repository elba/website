use actix_web::middleware::cors::CorsBuilder;

use crate::controller;
use crate::util::async_await::compat;
use crate::AppState;

pub fn router(cors: &mut CorsBuilder<AppState>) {
    cors.resource("/api/v1/users/login", |r| {
        r.get()
            .with(compat(controller::users::login::login_by_access_token))
    }).resource("/api/v1/users/login/oauth", |r| {
        r.get()
            .with(compat(controller::users::login::login_by_oauth))
    }).resource("/api/v1/users/login/oauth/callback", |r| {
        r.get()
            .with(compat(controller::users::login::login_by_oauth_callback))
    }).resource("/api/v1/users/logout", |r| {
        r.get().with(compat(controller::users::login::logout))
    }).resource("/api/v1/users/metadata", |r| {
        r.get()
            .with(compat(controller::users::metadata::show_user_self))
    }).resource("/api/v1/users/{id}/metadata", |r| {
        r.get().with(compat(controller::users::metadata::show_user))
    }).resource("/api/v1/users/tokens", |r| {
        r.get().with(compat(controller::users::token::list_tokens))
    }).resource("/api/v1/users/tokens/create", |r| {
        r.put().with(compat(controller::users::token::create_token))
    }).resource("/api/v1/users/tokens/{token_id}", |r| {
        r.delete()
            .with(compat(controller::users::token::remove_token))
    }).resource("/api/v1/packages/search", |r| {
        r.get().with(compat(controller::packages::search))
    }).resource("/api/v1/packages/global_stats", |r| {
        r.get()
            .with(compat(controller::packages::download::global_stats))
    }).resource("/api/v1/packages/groups", |r| {
        r.get()
            .with(compat(controller::packages::metadata::list_groups))
    }).resource("/api/v1/packages/{group}/metadata", |r| {
        r.get()
            .with(compat(controller::packages::metadata::show_group))
    }).resource("/api/v1/packages/{group}/packages", |r| {
        r.get()
            .with(compat(controller::packages::metadata::list_packages))
    }).resource("/api/v1/packages/{group}/{package}/metadata", |r| {
        r.get()
            .with(compat(controller::packages::metadata::show_package))
    }).resource("/api/v1/packages/{group}/{package}/versions", |r| {
        r.get()
            .with(compat(controller::packages::metadata::list_versions))
    }).resource(
        "/api/v1/packages/{group}/{package}/{version}/metadata",
        |r| {
            r.get()
                .with(compat(controller::packages::metadata::show_version))
        },
    ).resource("/api/v1/packages/{group}/{package}/{version}/readme", |r| {
        r.get()
            .with(compat(controller::packages::metadata::show_readme))
    }).resource(
        "/api/v1/packages/{group}/{package}/{version}/dependencies",
        |r| {
            r.get()
                .with(compat(controller::packages::metadata::list_dependencies))
        },
    ).resource(
        "/api/v1/packages/{group}/{package}/{version}/download",
        |r| {
            r.get()
                .with(compat(controller::packages::download::download))
        },
    ).resource("/api/v1/packages/{group}/{package}/{version}/yank", |r| {
        r.patch().with(compat(controller::packages::yank))
    }).resource(
        "/api/v1/packages/{group}/{package}/{version}/publish",
        |r| r.put().with(compat(controller::packages::publish)),
    ).resource(
        "/api/v1/packages/{group}/{package}/{version}/download_stats",
        |r| {
            r.get()
                .with(compat(controller::packages::download::download_stats))
        },
    ).resource(
        "/api/v1/packages/{group}/{package}/{version}/download_graph",
        |r| {
            r.get()
                .with(compat(controller::packages::download::download_graph))
        },
    );
}
