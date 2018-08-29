use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use actix_web::*;
use bytes::Bytes;
use elba::package::manifest::{DepReq, Manifest};
use failure::Error;
use futures::prelude::*;
use semver;
use tar::Archive;

use super::model::*;
use crate::util::response::report_error;
use crate::{AppState, CONFIG};

#[derive(Deserialize, Clone)]
pub struct PublishReq {
    pub package_group_name: String,
    pub package_name: String,
    pub semver: semver::Version,
    pub token: String,
}

pub fn publish(
    (query, state, req): (Query<PublishReq>, State<AppState>, HttpRequest<AppState>),
) -> impl Responder {
    let package_version = PackageVersion {
        name: PackageName::new(&query.package_group_name, &query.package_name),
        semver: query.semver.to_string(),
    };

    let verify_spec = state
        .db
        .send(VerifyVersion {
            package: package_version.clone(),
            token: query.token.clone(),
        }).from_err::<Error>()
        .flatten();

    let receive_body =
        verify_spec.and_then(move |()| req.body().limit(CONFIG.max_upload_size).from_err());

    let publish_and_save = receive_body
        .and_then(move |bytes: Bytes| {
            let manifest = read_manifest(&bytes)?;
            verify_manifest(&query, &manifest)?;

            let deps = deps_in_manifest(&manifest)?;

            let publish = state
                .db
                .send(PublishVersion {
                    package: package_version.clone(),
                    // TODO: read description from manifest
                    description: None,
                    dependencies: deps.clone(),
                    token: query.token.clone(),
                    bytes,
                }).from_err::<Error>()
                .flatten();

            Ok(publish)
        }).flatten();

    publish_and_save
        .map(|()| HttpResponse::Ok().finish())
        .or_else(report_error)
        .responder()
}

fn read_manifest(bytes: &[u8]) -> Result<Manifest, Error> {
    let mut archive = Archive::new(bytes);
    let mut entry = archive
        .entries()?
        .filter_map(Result::ok)
        .find(|entry| match entry.path() {
            Ok(ref path) if *path == Path::new("elba.toml") => true,
            _ => false,
        }).ok_or_else(|| format_err!("Manifest not found in archive."))?;

    let mut buffer = String::new();
    entry.read_to_string(&mut buffer)?;
    let manifest = Manifest::from_str(&buffer)?;

    Ok(manifest)
}

fn verify_manifest(req: &PublishReq, manifest: &Manifest) -> Result<(), Error> {
    if manifest.package.name.group() != req.package_group_name {
        return Err(format_err!("Package group name mismatched."));
    }

    if manifest.package.name.name() != req.package_name {
        return Err(format_err!("Package name mismatched."));
    }

    if manifest.package.version != req.semver {
        return Err(format_err!("Package version mismatched."));
    }

    // TODO: check outer index reference

    Ok(())
}

fn deps_in_manifest(manifest: &Manifest) -> Result<Vec<(DependencyReq)>, Error> {
    let mut deps = Vec::new();

    for (name, ver_req) in manifest.dependencies.iter() {
        let version_req = match ver_req {
            DepReq::Registry(constrain) => constrain.to_string(),
            _ => {
                return Err(format_err!(
                    "Package contains non-index dependency {}/{}.",
                    name.group(),
                    name.name()
                ))
            }
        };

        deps.push(DependencyReq {
            name: PackageName::new(name.group(), name.name()),
            version_req,
        });
    }

    Ok(deps)
}
