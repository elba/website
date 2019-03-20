use actix_web::*;
use failure::Error;
use futures::prelude::*;

use crate::model::users::*;
use crate::util::error::report_error;
use crate::AppState;

use super::*;

pub fn show_user((path, state): (Path<UserReq>, State<AppState>)) -> impl Responder {
    let lookup_user = state
        .db
        .send(LookupUser { id: path.id })
        .from_err::<Error>()
        .flatten();

    lookup_user
        .map(move |user| {
            let user_meta = UserView::from(user);

            #[derive(Serialize)]
            struct R {
                user: UserView,
            }

            Ok(HttpResponse::Ok().json(R { user: user_meta }))
        }).flatten()
        .or_else(report_error)
        .responder()
}
