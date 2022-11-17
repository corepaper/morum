use actix_web::{ResponseError, http::StatusCode, HttpResponse, body::BoxBody};
use derive_more::Display;

pub enum Error {

}

#[derive(Debug, Display)]
pub enum UserError {

}

impl From<Error> for UserError {
    fn from(err: Error) -> Self {
        match err {

        }
    }
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        unimplemented!()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        unimplemented!()
    }
}
