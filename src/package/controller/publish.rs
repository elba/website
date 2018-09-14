use std::convert::TryFrom;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use actix_web::{self, *};
use bytes::Bytes;
use elba::package::manifest::{DepReq, Manifest};
use failure::Error;
use futures::{future, prelude::*};
use tar::Archive;

use super::PackageVersionReq;

use crate::package::model::*;
use crate::util::error::report_error;
use crate::{AppState, CONFIG};

#[derive(Deserialize, Clone)]
pub struct PublishReq {
    pub token: String,
}

pub fn publish(
    (path, query, state, req): (
        actix_web::Path<PackageVersionReq>,
        Query<PublishReq>,
        State<AppState>,
        HttpRequest<AppState>,
    ),
) -> impl Responder {
    let package_version = match PackageVersion::try_from(path.clone()) {
        Ok(package_version) => package_version,
        Err(err) => {
            let error: Box<Future<Item = _, Error = _>> = Box::new(future::err(err));
            return error;
        }
    };

    let verify_spec = state
        .db
        .send(VerifyVersion {
            package: package_version.clone(),
            token: query.token.clone(),
        }).from_err::<Error>()
        .flatten();

    let receive_body = verify_spec.and_then(move |()| {
        info!("Receiving tarball");
        req.body().limit(CONFIG.max_upload_size).from_err()
    });

    let publish_and_save = receive_body
        .and_then(move |bytes: Bytes| {
            let manifest = read_manifest(&bytes)?;
            verify_manifest(&path, &manifest)?;

            let deps = deps_in_manifest(&manifest)?;
            let readme_file = read_readme(
                &bytes,
                manifest
                    .package
                    .readme
                    .as_ref()
                    .map(|subpath| subpath.0.as_path()),
            )?;

            let publish = state
                .db
                .send(PublishVersion {
                    package: package_version.clone(),
                    package_info: manifest.package.clone(),
                    readme_file,
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
        }).ok_or_else(|| human!("Manifest not found in archive"))?;

    let mut buffer = String::new();
    entry.read_to_string(&mut buffer)?;
    let manifest = Manifest::from_str(&buffer)?;

    Ok(manifest)
}

fn read_readme(bytes: &[u8], subpath: Option<&Path>) -> Result<Option<String>, Error> {
    let mut archive = Archive::new(bytes);
    let entry = archive.entries()?.filter_map(Result::ok).find(|entry| {
        if let Ok(path) = entry.path() {
            if let Some(subpath) = subpath {
                if path == subpath {
                    return true;
                }
            }
            if let Some(file_stem) = path.file_stem() {
                return &file_stem.to_string_lossy().to_uppercase() == "README";
            }
            false
        } else {
            false
        }
    });

    if let Some(mut entry) = entry {
        if entry.header().entry_size()? > 2 * 1024 * 1024 {
            return Ok(None);
        }

        let mut buffer = String::new();
        entry.read_to_string(&mut buffer)?;

        Ok(Some(buffer))
    } else {
        Ok(None)
    }
}

fn verify_manifest(req: &PackageVersionReq, manifest: &Manifest) -> Result<(), Error> {
    if manifest.package.name.group() != req.group {
        return Err(human!("Package group name mismatched"));
    }

    if manifest.package.name.name() != req.package {
        return Err(human!("Package name mismatched"));
    }

    if manifest.package.version != req.version {
        return Err(human!("Package version mismatched"));
    }

    // TODO: check outer index reference

    Ok(())
}

fn deps_in_manifest(manifest: &Manifest) -> Result<Vec<(DependencyReq)>, Error> {
    let mut deps = Vec::new();

    for (name, ver_req) in manifest.dependencies.iter() {
        let version_req = match ver_req {
            DepReq::Registry(constrain) => constrain.clone(),
            _ => {
                return Err(human!(
                    "Package contains non-index dependency {}",
                    name.as_str()
                ))
            }
        };

        deps.push(DependencyReq {
            name: name.clone(),
            version_req,
        });
    }

    Ok(deps)
}
