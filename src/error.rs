use actix_web::{ResponseError, http::StatusCode, HttpResponse, body::BoxBody};
use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json Web Token related errors")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Login credential is invalid")]
    InvalidLoginCredential,
}

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
            Error::Jwt(_) => Self::Internal,

            Error::InvalidLoginCredential => Self::InvalidLoginCredential
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
