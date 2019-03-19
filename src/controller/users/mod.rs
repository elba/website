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
    pub token: String,
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
