use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use actix_web::*;
use bytes::Bytes;
use elba::package::{
    manifest::{DepReq, Manifest},
    Name,
};
use failure::Error;
use futures::{future, prelude::*};
use semver;
use tar::Archive;

use crate::package::model::*;
use crate::util::error::report_error;
use crate::{AppState, CONFIG};

#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub q: String,
}

// pub fn search((query, state): (Query<SearchReq>, State<AppState>)) -> impl Responder {
//     unimplemented!()
// }
