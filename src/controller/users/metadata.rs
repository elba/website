use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::users::*;
use crate::AppState;

use super::*;

pub async fn show_user(
    (path, state): (Path<UserReq>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let user = await!(state.db.send(LookupUser { id: path.id }))??;

    let user_meta = UserView::from(user);

    #[derive(Serialize)]
    struct R {
        user: UserView,
    }

    Ok(HttpResponse::Ok().json(R { user: user_meta }))
}
