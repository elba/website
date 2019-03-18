use actix_web::*;
use failure::Error;
use futures::prelude::*;

use crate::user::model::*;
use crate::util::error::report_error;
use crate::AppState;

use super::*;

pub fn show_user((path, state): (Path<UserView>, State<AppState>)) -> impl Responder {
    let lookup_user = state
        .db
        .send(LookupUser { id: path.id })
        .from_err::<Error>()
        .flatten();

    lookup_user
        .map(move |user| {
            let user_meta = UserMetadata::from(user);
            Ok(HttpResponse::Ok().json(user_meta))
        })
        .flatten()
        .or_else(report_error)
        .responder()
}
