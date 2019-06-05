use actix_web::HttpResponse;
use failure::{Context, Error, Fail};
use std::fmt::{self, Display};

/// For user error, this responds `400 Bad Request` along with
/// an error description in json.
/// For internal error, this responds `500 Internal Error` and
/// logs the error chain.
/// Any error that does not have HumanError in its error chain
/// is considered as an initernal error.
pub fn report_error(error: Error) -> HttpResponse {
    fn find_human_error(error: &Error) -> Option<&HumanError> {
        let mut human_error = None;
        for fail in error.iter_chain() {
            if let Some(error) = fail.downcast_ref::<HumanError>() {
                human_error = Some(error);
                break;
            } else if let Some(error) = fail.downcast_ref::<Context<Error>>() {
                return find_human_error(error.get_context());
            }
        }
        human_error
    }

    if let Some(human_error) = find_human_error(&error) {
        HttpResponse::BadRequest().body(format!(
            "{{\"error\":\"{}\",\"description\":\"{}\"}}",
            human_error.reason.tag(),
            human_error.message
        ))
    } else {
        error!("Internal error: {:?}", error);
        HttpResponse::InternalServerError().body("registry internal error")
    }
}

#[derive(Debug)]
pub enum Reason {
    InvalidRequest,
    InvalidFormat,
    InvalidManifest,
    NoPermission,
    UserNotFound,
    TokenNotFound,
    PackageNotFound,
    DependencyNotFound,
}

impl Reason {
    pub fn tag(&self) -> &'static str {
        match self {
            Reason::InvalidRequest => "invalid_request",
            Reason::InvalidFormat => "invalid_format",
            Reason::InvalidManifest => "invalid_manifest",
            Reason::NoPermission => "no_permission",
            Reason::UserNotFound => "user_not_found",
            Reason::TokenNotFound => "token_not_found",
            Reason::PackageNotFound => "package_not_found",
            Reason::DependencyNotFound => "dependency_not_found",
        }
    }
}

#[derive(Debug)]
pub struct HumanError {
    pub reason: Reason,
    pub message: String,
}

impl HumanError {
    pub fn into_error(self) -> Error {
        Error::from(self)
    }
}

impl Display for HumanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Fail for HumanError {
    fn cause(&self) -> Option<&Fail> {
        None
    }
}

#[macro_export]
macro_rules! human {
    ($reason:expr, $($arg:tt)*) => ({
        let message = format!($($arg)*);
        $crate::util::error::HumanError { reason: $reason, message }.into_error()
    })
}
