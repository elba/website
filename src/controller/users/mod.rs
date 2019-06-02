pub mod metadata;
pub mod token;

pub mod login;

use chrono::NaiveDateTime;

use crate::model::users::{AccessToken, User};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserReq {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct AccessTokenReq {
    token_id: i32,
}

#[derive(Serialize, Clone)]
pub struct UserView {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct AccessTokenView {
    pub id: i32,
    pub token: Option<String>,
    pub token_partial: String,
    #[serde(with = "crate::util::rfc3339")]
    pub created_at: NaiveDateTime,
}

impl AccessTokenView {
    pub fn hide_token(self) -> Self {
        Self {
            token: None,
            ..self
        }
    }
}

impl From<AccessToken> for AccessTokenView {
    fn from(access_token: AccessToken) -> AccessTokenView {
        let mut token_partial = access_token.token.clone();
        token_partial.replace_range(4..access_token.token.len() - 4, "....");

        AccessTokenView {
            id: access_token.id,
            token: Some(access_token.token),
            token_partial,
            created_at: access_token.created_at,
        }
    }
}

impl From<User> for UserView {
    fn from(user: User) -> UserView {
        UserView {
            id: user.id,
            email: user.email,
            name: user.gh_name,
            avatar: user.gh_avatar,
        }
    }
}
