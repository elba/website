use actix_web::HttpResponse;
use failure::{Error, Fail};
use std::fmt::{self, Display, Write};

/// For user error, this responds `400 Bad Request` along with
/// an error description in json.
/// For internal error, this responds `500 Internal Error` and
/// logs the error chain.
/// Any error that does not have HumanError in its error chain
/// is considered as an initernal error.
pub fn report_error(error: Error) -> Result<HttpResponse, Error> {
    let human_error = error
        .iter_chain()
        .filter_map(|fail| fail.downcast_ref::<HumanError>())
        .nth(0);

    if let Some(human_error) = human_error {
        Ok(HttpResponse::BadRequest().body(format!("{{error:\"{}\"}}", human_error.message)))
    } else {
        let mut error_chain_str = String::new();
        error
            .iter_chain()
            .for_each(|fail| write!(error_chain_str, "\n\t- {}", fail).unwrap());
        error!("Internal error:{}", error_chain_str);

        Ok(HttpResponse::InternalServerError().finish())
    }
}

#[derive(Debug)]
pub struct HumanError {
    pub message: String,
    pub failure: Option<Error>,
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
        self.failure.as_ref().map(|err| err.as_fail())
    }
}

#[macro_export]
macro_rules! human {
    ($($arg:tt)*) => ({
        let message = format!($($arg)*);
        $crate::util::error::HumanError { message, failure: None }.into_error()
    })
}
