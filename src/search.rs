use std::str::FromStr;

use actix_web::{client, HttpMessage};
use elba::package::{manifest::PackageInfo, Name as PackageName};
use failure::Error;
use futures::future::{self, Future, IntoFuture};
use itertools::Itertools;
use reqwest::{Client, StatusCode};

use crate::CONFIG;

#[derive(Serialize)]
struct UpdateRequest {
    id: String,
    content: String,
}

pub fn update_package(package_info: &PackageInfo) -> Result<(), Error> {
    info!("updating search index");

    let client = Client::new();
    let res = client
        .post(&format!("{}/add", CONFIG.search_engine_url))
        .json(&UpdateRequest {
            id: package_info.name.as_normalized().to_owned(),
            content: vec![package_info.name.group(), package_info.name.name()]
                .into_iter()
                .chain(package_info.keywords.iter().map(|s| s.as_str()))
                .join(" "),
        }).send()?;

    if res.status() == StatusCode::OK {
        Ok(())
    } else {
        Err(format_err!("failed to update search index. res: {:?}", res))
    }
}

#[derive(Serialize)]
struct SearchRequest {
    query: String,
}

pub fn search_package(query: String) -> Box<Future<Item = Vec<PackageName>, Error = Error>> {
    let future = client::get(format!("{}/search", CONFIG.search_engine_url))
        .json(SearchRequest { query })
        .map_err(|err| format_err!("{:?}", err))
        .into_future()
        .and_then(|req| req.send().from_err::<Error>())
        .and_then(|res| -> Box<Future<Item = Vec<String>, Error = Error>> {
            if res.status() == StatusCode::OK {
                Box::new(res.json::<Vec<String>>().from_err::<Error>())
            } else {
                Box::new(future::err(format_err!(
                    "failed to update search index. res: {:?}",
                    res
                )))
            }
        }).and_then(|res| {
            Ok(res
                .iter()
                .filter_map(|name| PackageName::from_str(name).ok())
                .collect::<Vec<_>>())
        });

    Box::new(future)
}
