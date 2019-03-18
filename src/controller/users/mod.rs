pub mod metadata;

mod login;
// mod token;

pub use login::login;
// pub use token::create_token;

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
