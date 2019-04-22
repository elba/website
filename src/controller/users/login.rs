use actix_web::middleware::identity::RequestIdentity;
use actix_web::{HttpRequest, HttpResponse, Query, State};
use failure::Error;
use tokio_async_await::await;

use crate::login::{self, LoginByAccessToken, LoginByOAuth};
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginByAccessTokenReq {
    gh_access_token: String,
}

#[derive(Deserialize)]
pub struct OAuthCallbackReq {
    code: String,
}

pub async fn login_by_access_token(
    (query, state, req): (
        Query<LoginByAccessTokenReq>,
        State<AppState>,
        HttpRequest<AppState>,
    ),
) -> Result<HttpResponse, Error> {
    let user_id = await!(state.login.send(LoginByAccessToken {
        gh_access_token: query.into_inner().gh_access_token
    }))??;

    req.remember(user_id.to_string());

    Ok(HttpResponse::Ok().finish())
}

pub async fn login_by_oauth((): ()) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::TemporaryRedirect()
        .header("Location", login::get_oauth_url()?)
        .finish())
}

pub async fn login_by_oauth_callback(
    (query, state, req): (
        Query<OAuthCallbackReq>,
        State<AppState>,
        HttpRequest<AppState>,
    ),
) -> Result<HttpResponse, Error> {
    let user_id = await!(state.login.send(LoginByOAuth {
        code: query.into_inner().code
    }))??;

    req.remember(user_id.to_string());

    Ok(HttpResponse::Ok().finish())
}
