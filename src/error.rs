use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json Web Token related errors")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Yaml parse error")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Login credential is invalid")]
    InvalidLoginCredential,
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        match err {
            Error::Io(err) => err,
            err => std::io::Error::new(std::io::ErrorKind::Other, err.to_string()),
        }
    }
}
