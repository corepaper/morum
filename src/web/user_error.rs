use crate::Error;
use thiserror::Error;
use axum::{http::StatusCode, response::{Response, IntoResponse, Redirect}};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Internal error")]
    Internal,
    #[error("Login credential is invalid")]
    InvalidLoginCredential,
    #[error("Already logged in")]
    AlreadyLoggedIn,
}

impl From<Error> for UserError {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidLoginCredential => Self::InvalidLoginCredential,

            _ => Self::Internal,
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response  {
        match self {
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string()).into_response(),
            Self::InvalidLoginCredential => (StatusCode::UNAUTHORIZED, "invalid login".to_string()).into_response(),
            Self::AlreadyLoggedIn => Redirect::to("/").into_response(),
        }
    }
}
