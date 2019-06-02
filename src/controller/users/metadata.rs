use actix_web::middleware::identity::RequestIdentity;
use actix_web::*;
use failure::Error;
use tokio_async_await::await;

use crate::model::users::*;
use crate::util::error::Reason;
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

pub async fn show_user_self(
    (state, req): (State<AppState>, HttpRequest<AppState>),
) -> Result<HttpResponse, Error> {
    let user_id: i32 = match req.identity() {
        Some(user_id) => user_id.parse().unwrap(),
        None => {
            return Err(human!(Reason::NoPermission, "please login first"));
        }
    };

    let user = await!(state.db.send(LookupUser { id: user_id }))??;

    let user_meta = UserView::from(user);

    #[derive(Serialize)]
    struct R {
        user: UserView,
    }

    Ok(HttpResponse::Ok().json(R { user: user_meta }))
}
