use actix_web::HttpResponse;
use failure::Error;

#[derive(Serialize)]
struct ErrorReport {
    pub error: String,
}

pub fn report_error(err: Error) -> Result<HttpResponse, Error> {
    let report = ErrorReport {
        error: format!("{}", err.as_fail()),
    };
    Ok(HttpResponse::BadRequest().json(report))
}
