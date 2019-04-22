pub mod metadata;
pub mod token;

pub mod login;

use crate::model::users::User;

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
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct AccessTokenView {
    pub id: i32,
    pub token: Option<String>,
    pub token_partial: String,
}

impl AccessTokenView {
    pub fn new(id: i32, token: String) -> Self {
        let mut token_partial = token.clone();
        token_partial.replace_range(4..token.len() - 4, "********");

        Self {
            id,
            token: Some(token),
            token_partial,
        }
    }

    pub fn hide_token(self) -> Self {
        Self {
            token: None,
            ..self
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
