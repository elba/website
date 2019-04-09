use actix_web::middleware::identity::RequestIdentity;
use actix_web::{client, http::StatusCode, HttpMessage, HttpRequest, HttpResponse, Query, State};
use base64;
use failure::Error;
use tokio_async_await::await;

use crate::model::users::CreateUserOrLogin;
use crate::util::error::Reason;
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

pub async fn login(
    (query, state, req): (Query<LoginReq>, State<AppState>, HttpRequest<AppState>),
) -> Result<HttpResponse, Error> {
    let auth = base64::encode(&format!("{}:{}", query.gh_name, query.gh_access_token));

    let github_response = await!(
        client::get("https://api.github.com/user")
            .header("Authorization", format!("Basic {}", auth).as_str())
            .finish()
            .map_err(|err| format_err!("{:?}", err))?
            .send()
    )?;

    if github_response.status() != StatusCode::OK {
        return Err(human!(
            Reason::UserNotFound,
            "Bad username or access token to Github"
        ));
    }

    let response_json: GithubRes = await!(github_response.json())?;

    let user = await!(state.db.send(CreateUserOrLogin {
        email: response_json.email,
        gh_id: response_json.id,
        gh_name: query.gh_name.clone(),
        gh_access_token: query.gh_access_token.clone(),
        gh_avatar: response_json.avatar_url,
    }))??;

    req.remember(user.id.to_string());

    Ok(HttpResponse::Ok().finish())
}
