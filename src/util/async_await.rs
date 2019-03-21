use actix_web::{FutureResponse, HttpResponse};
use failure::Error;
use futures::{future, Future as _};
use tokio_async_await::compat::backward::Compat;

use std::future::Future;

use crate::util::error::report_error;

#[inline]
pub fn compat<F, Fut, Arg>(f: F) -> impl Fn(Arg) -> FutureResponse<HttpResponse>
where
    F: Fn(Arg) -> Fut,
    Fut: Future<Output = Result<HttpResponse, Error>> + 'static,
{
    move |arg| Box::new(Compat::new(f(arg)).or_else(|err| future::ok(report_error(err.into()))))
}
