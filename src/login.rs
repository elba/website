use actix::prelude::*;
use actix_web::{client, http::StatusCode, FutureResponse, HttpMessage};
use failure::Error;
use tokio_async_await::await;

use crate::database::Database;
use crate::model::users::CreateUserOrLogin;
use crate::util::async_await::compat_future;
use crate::util::error::Reason;
use crate::CONFIG;

#[derive(Clone)]
pub struct GhOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

pub struct GhLogin {
    pub db: Addr<Database>,
}

impl Actor for GhLogin {
    type Context = Context<Self>;
}

pub struct LoginByAccessToken {
    pub gh_access_token: String,
}

pub struct LoginByOAuth {
    pub code: String,
}

impl Message for LoginByAccessToken {
    type Result = Result<i32, Error>;
}

impl Message for LoginByOAuth {
    type Result = Result<i32, Error>;
}

#[derive(Deserialize)]
struct GithubUserRes {
    id: i32,
    name: String,
    email: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Deserialize)]
struct GithubGetAccessTokenRes {
    access_token: String,
}

impl Handler<LoginByAccessToken> for GhLogin {
    type Result = FutureResponse<i32, Error>;

    fn handle(&mut self, msg: LoginByAccessToken, _: &mut Self::Context) -> Self::Result {
        let db = self.db.clone();

        compat_future(
            async move {
                let github_response = await!(client::get("https://api.github.com/user")
                    .basic_auth("token", Some(&msg.gh_access_token))
                    .finish()
                    .map_err(|err| format_err!("{:?}", err))?
                    .send())?;

                if github_response.status() != StatusCode::OK {
                    return Err(human!(
                        Reason::UserNotFound,
                        "Bad username or access token to Github"
                    ));
                }

                let response_json: GithubUserRes = await!(github_response.json())?;

                let user = await!(db.send(CreateUserOrLogin {
                    email: response_json.email,
                    gh_id: response_json.id,
                    gh_name: response_json.name,
                    gh_access_token: msg.gh_access_token,
                    gh_avatar: response_json.avatar_url,
                }))??;

                Ok(user.id)
            },
        )
    }
}

impl Handler<LoginByOAuth> for GhLogin {
    type Result = FutureResponse<i32, Error>;

    fn handle(&mut self, msg: LoginByOAuth, ctx: &mut Self::Context) -> Self::Result {
        let self_addr = ctx.address().clone();

        compat_future(
            async move {
                match &CONFIG.gh_oauth_config {
                    Some(GhOAuthConfig {
                        client_id,
                        client_secret,
                    }) => {
                        let github_response = await!(client::get(format!(
                            "https://github.com/login/oauth/access_token\
                             &client_id={}\
                             &client_secret={}\
                             &code={}\
                             &accept=json",
                            client_id, client_secret, msg.code
                        ))
                        .finish()
                        .map_err(|err| format_err!("{:?}", err))?
                        .send())?;

                        let response_json: GithubGetAccessTokenRes =
                            await!(github_response.json())?;

                        let user_id = await!(self_addr.send(LoginByAccessToken {
                            gh_access_token: response_json.access_token
                        }))??;

                        Ok(user_id)
                    }
                    None => Err(human!(
                        Reason::InvalidRequest,
                        "Github OAuth is disabled by admin."
                    )),
                }
            },
        )
    }
}

pub fn get_oauth_url() -> Result<String, Error> {
    match &CONFIG.gh_oauth_config {
        Some(GhOAuthConfig { client_id, .. }) => Ok(format!(
            "https://github.com/login/oauth/authorize?scope=user:email\
             &client_id={}\
             &redirect_uri={}/api/v1/users/login/oauth/callback",
            &client_id, &CONFIG.registry.url
        )),
        None => Err(human!(
            Reason::InvalidRequest,
            "Github OAuth is disabled by admin."
        )),
    }
}
