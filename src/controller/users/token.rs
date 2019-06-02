use actix_web::middleware::identity::RequestIdentity;
use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::users::*;
use crate::util::error::Reason;
use crate::AppState;

use super::*;

pub async fn list_tokens(
    (state, req): (State<AppState>, HttpRequest<AppState>),
) -> Result<HttpResponse, Error> {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            return Err(human!(Reason::NoPermission, "please login first"));
        }
    };

    let access_tokens = await!(state.db.send(ListAccessTokens { user_id }))??;
    let access_tokens = access_tokens
        .into_iter()
        .map(|access_token| AccessTokenView::from(access_token).hide_token())
        .collect();

    #[derive(Serialize)]
    struct R {
        tokens: Vec<AccessTokenView>,
    }

    Ok(HttpResponse::Ok().json(R {
        tokens: access_tokens,
    }))
}

pub async fn create_token(
    (state, req): (State<AppState>, HttpRequest<AppState>),
) -> Result<HttpResponse, Error> {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            return Err(human!(Reason::NoPermission, "please login first"));
        }
    };

    let access_token = await!(state.db.send(CreateAccessToken { user_id }))??;
    let access_token = AccessTokenView::from(access_token);

    #[derive(Serialize)]
    struct R {
        token: AccessTokenView,
    }

    Ok(HttpResponse::Ok().json(R {
        token: access_token,
    }))
}

pub async fn remove_token(
    (path, state, req): (Path<AccessTokenReq>, State<AppState>, HttpRequest<AppState>),
) -> Result<HttpResponse, Error> {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            return Err(human!(Reason::NoPermission, "please login first"));
        }
    };

    await!(state.db.send(RemoveAccessToken {
        user_id,
        access_token_id: path.token_id,
    }))??;

    Ok(HttpResponse::Ok().finish())
}
