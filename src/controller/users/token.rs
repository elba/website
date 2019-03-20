use actix_web::middleware::identity::RequestIdentity;
use actix_web::{AsyncResponder, HttpRequest, HttpResponse, Path, Responder, State};
use failure::Error;
use futures::{future, prelude::*};

use crate::model::users::*;
use crate::util::error::{report_error, Reason};
use crate::AppState;

use super::*;

pub fn list_tokens((state, req): (State<AppState>, HttpRequest<AppState>)) -> impl Responder {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            let err = human!(Reason::InvalidManifest, "Please login first");
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let list_access_tokens = state
        .db
        .send(ListAccessTokens { user_id })
        .from_err::<Error>()
        .flatten();

    list_access_tokens
        .map(move |access_tokens: Vec<AccessToken>| {
            let tokens = access_tokens
                .into_iter()
                .map(|access_token| {
                    AccessTokenView::new(access_token.id, access_token.token).hide_token()
                }).collect();

            #[derive(Serialize)]
            struct R {
                tokens: Vec<AccessTokenView>,
            }

            HttpResponse::Ok().json(R { tokens })
        }).or_else(report_error)
        .responder()
}

pub fn create_token((state, req): (State<AppState>, HttpRequest<AppState>)) -> impl Responder {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            let err = human!(Reason::InvalidManifest, "Please login first");
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let create_access_token = state
        .db
        .send(CreateAccessToken { user_id })
        .from_err::<Error>()
        .flatten();

    create_access_token
        .map(move |access_token: AccessToken| {
            #[derive(Serialize)]
            struct R {
                token: AccessTokenView,
            }

            HttpResponse::Ok().json(R {
                token: AccessTokenView::new(access_token.id, access_token.token),
            })
        }).or_else(report_error)
        .responder()
}

pub fn remove_token(
    (path, state, req): (Path<AccessTokenReq>, State<AppState>, HttpRequest<AppState>),
) -> impl Responder {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            let err = human!(Reason::InvalidManifest, "Please login first");
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let remove_access_token = state
        .db
        .send(RemoveAccessToken {
            user_id,
            access_token_id: path.token_id,
        }).from_err::<Error>()
        .flatten();

    remove_access_token
        .map(move |()| HttpResponse::Ok().finish())
        .or_else(report_error)
        .responder()
}
