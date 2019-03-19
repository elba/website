use actix_web::middleware::identity::RequestIdentity;
use actix_web::{
    client, http::StatusCode, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Query,
    Responder, State,
};
use base64;
use failure::Error;
use futures::prelude::*;

use crate::model::users::{CreateUserOrLogin, User};
use crate::util::error::{report_error, Reason};
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginReq {
    gh_name: String,
    gh_access_token: String,
}

#[derive(Deserialize)]
struct GithubRes {
    id: i32,
    email: Option<String>,
    avatar_url: Option<String>,
}

pub fn login(
    (query, state, req): (Query<LoginReq>, State<AppState>, HttpRequest<AppState>),
) -> impl Responder {
    let auth = base64::encode(&format!("{}:{}", query.gh_name, query.gh_access_token));

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

    let create_user_or_login = parse_response.and_then(move |json: GithubRes| {
        state
            .db
            .send(CreateUserOrLogin {
                email: json.email,
                gh_id: json.id,
                gh_name: query.gh_name.clone(),
                gh_access_token: query.gh_access_token.clone(),
                gh_avatar: json.avatar_url,
            }).flatten()
    });

    create_user_or_login
        .map(move |user: User| {
            req.remember(user.id.to_string());

            HttpResponse::Ok().finish()
        }).or_else(report_error)
        .responder()
}
