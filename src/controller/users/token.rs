use actix_web::{
    client, http::StatusCode, AsyncResponder, HttpMessage, HttpResponse, Query, Responder, State,
};
use base64;
use failure::Error;
use futures::prelude::*;

use crate::model::users::{CreateUser, User};
use crate::util::error::{report_error, Reason};
use crate::AppState;

// TODO: check login
pub fn create_token((req, state): (Query<LoginReq>, State<AppState>)) -> impl Responder {
    let auth = base64::encode(&format!("{}:{}", req.gh_name, req.gh_access_token));

    let login_github = client::get("https://api.github.com/user")
        .header("Authorization", format!("Basic {}", auth).as_str())
        .finish()
        .unwrap()
        .send()
        .from_err::<Error>();

    let parse_response = login_github
        .and_then(|res| {
            if res.status() != StatusCode::OK {
                return Err(human!(
                    Reason::UserNotFound,
                    "Bad username or access token to Github"
                ));
            }

            Ok(res.json().from_err())
        }).flatten();

    let create_user = parse_response.and_then(move |json: GithubRes| {
        state
            .db
            .send(CreateUser {
                email: json.email,
                gh_id: json.id,
                gh_name: req.gh_name.clone(),
                gh_access_token: req.gh_access_token.clone(),
                gh_avatar: json.avatar_url,
            }).flatten()
    });

    create_user
        .map(|_: User| HttpResponse::Ok().finish())
        .or_else(report_error)
        .responder()
}
