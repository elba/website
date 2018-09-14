use actix_web::{
    client, http::StatusCode, AsyncResponder, HttpMessage, HttpResponse, Query, Responder, State,
};
use base64;
use chrono::offset::Utc;
use failure::Error;
use futures::prelude::*;

use super::model::{CreateUser, User};
use crate::util::error::report_error;
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginReq {
    gh_name: String,
    gh_access_token: String,
}

#[derive(Serialize)]
pub struct LoginRes {
    token: String,
}

#[derive(Deserialize)]
struct GithubRes {
    id: i32,
    email: Option<String>,
    avatar_url: Option<String>,
}

pub fn login((req, state): (Query<LoginReq>, State<AppState>)) -> impl Responder {
    let auth = base64::encode(&format!("{}:{}", req.gh_name, req.gh_access_token));

    let login_github = client::get("https://api.github.com/user")
        .header("Authorization", format!("Basic {}", auth).as_str())
        .finish()
        .unwrap()
        .send()
        .from_err::<Error>();

    let pares_response = login_github
        .and_then(|res| {
            if res.status() != StatusCode::OK {
                return Err(human!("Bad username or access token to Github"));
            }

            Ok(res.json().from_err())
        }).flatten();

    let create_user = pares_response.and_then(move |json: GithubRes| {
        state
            .db
            .send(CreateUser {
                email: json.email,
                gh_id: json.id,
                gh_name: req.gh_name.clone(),
                gh_access_token: req.gh_access_token.clone(),
                gh_avatar: json.avatar_url,
                last_used_at: Utc::now().naive_utc(),
            }).flatten()
    });

    create_user
        .map(|user: User| HttpResponse::Ok().json(LoginRes { token: user.token }))
        .or_else(report_error)
        .responder()
}
