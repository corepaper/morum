use crate::Error;
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
