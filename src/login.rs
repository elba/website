use actix::prelude::*;
use actix_web::FutureResponse;
use failure::Error;
use reqwest::r#async::Client;
use reqwest::StatusCode;
use std::sync::Arc;
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
    pub success_redirect_url: String,
}

pub struct GhLogin {
    db: Addr<Database>,
    client: Arc<Client>,
}

impl GhLogin {
    pub fn new(db: Addr<Database>) -> Self {
        GhLogin {
            db,
            client: Arc::new(Client::new()),
        }
    }
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
    login: String,
    name: Option<String>,
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
        let client = self.client.clone();

        compat_future(
            async move {
                let mut github_response = await!(
                    client
                        .get("https://api.github.com/user")
                        .header("Accept", "application/json")
                        .basic_auth("token", Some(&msg.gh_access_token))
                        .send()
                )?;

                if github_response.status() != StatusCode::OK {
                    return Err(human!(
                        Reason::UserNotFound,
                        "This access token to Github is invalid "
                    ));
                }

                let response_json: GithubUserRes = await!(github_response.json())?;

                let email = match response_json.email {
                    Some(email) => email,
                    None => {
                        return Err(human!(
                            Reason::InvalidRequest,
                            "This github account should have a public email"
                        ))
                    }
                };

                let user = await!(db.send(CreateUserOrLogin {
                    email: email,
                    gh_id: response_json.id,
                    gh_name: response_json.name.unwrap_or(response_json.login),
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
        let client = self.client.clone();

        compat_future(
            async move {
                match &CONFIG.gh_oauth_config {
                    Some(GhOAuthConfig {
                        client_id,
                        client_secret,
                        ..
                    }) => {
                        let mut github_response = await!(
                            client
                                .get(&format!(
                                    "https://github.com/login/oauth/access_token?\
                                     client_id={}\
                                     &client_secret={}\
                                     &code={}",
                                    client_id, client_secret, msg.code
                                )).header("Accept", "application/json")
                                .send()
                        )?;

                        if github_response.status() != StatusCode::OK {
                            return Err(format_err!("faild to retireve access token from Github"));
                        }

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
            "https://github.com/login/oauth/authorize?client_id={}",
            &client_id
        )),
        None => Err(human!(
            Reason::InvalidRequest,
            "Github OAuth is disabled by admin."
        )),
    }
}

pub fn get_success_redirect_url() -> Result<String, Error> {
    match &CONFIG.gh_oauth_config {
        Some(GhOAuthConfig {
            success_redirect_url,
            ..
        }) => Ok(success_redirect_url.clone()),
        None => Err(human!(
            Reason::InvalidRequest,
            "Github OAuth is disabled by admin."
        )),
    }
}
