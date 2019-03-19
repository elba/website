use actix_web::middleware::identity::RequestIdentity;
use actix_web::{AsyncResponder, HttpRequest, HttpResponse, Responder, State};
use failure::Error;
use futures::{future, prelude::*};

use crate::model::users::*;
use crate::util::error::{report_error, Reason};
use crate::AppState;

use super::*;

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
                token: AccessTokenMetadata,
            }

            HttpResponse::Ok().json(R {
                token: AccessTokenMetadata {
                    id: access_token.id,
                    token: access_token.token,
                },
            })
        }).or_else(report_error)
        .responder()
}
