pub mod metadata;
pub mod token;

mod login;

pub use login::login;

use crate::model::users::User;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserView {
    pub id: i32,
}

#[derive(Serialize, Clone)]
pub struct UserMetadata {
    pub id: i32,
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct AccessTokenMetadata {
    pub id: i32,
    pub token: Option<String>,
    pub token_partial: String,
}

impl AccessTokenMetadata {
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

impl From<User> for UserMetadata {
    fn from(user: User) -> UserMetadata {
        UserMetadata {
            id: user.id,
            email: user.email,
            name: user.gh_name,
            avatar: user.gh_avatar,
        }
    }
}
