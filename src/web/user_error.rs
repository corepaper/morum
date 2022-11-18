use crate::Error;
use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Internal error")]
    Internal,
    #[error("Login credential is invalid")]
    InvalidLoginCredential,
}

impl From<Error> for UserError {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidLoginCredential => Self::InvalidLoginCredential,

            _ => Self::Internal,
        }
    }
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Ok().finish()
    }
}
