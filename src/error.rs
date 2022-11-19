use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json Web Token related errors")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Yaml parse error")]
    Yaml(#[from] serde_yaml::Error),
    // #[error("Appservice error")]
    // Appservice(#[from] matrix_sdk_appservice::Error),
    #[error("Url parsing error")]
    UrlParse(#[from] url::ParseError),
    #[error("Ruma client error")]
    RumaClient(Box<dyn std::error::Error>),
    // #[error("Matrix error")]
    // Matrix(#[from] matrix_sdk::Error),
    // #[error("Matrix HTTP error")]
    // MatrixHttp(#[from] matrix_sdk::HttpError),
    // #[error("Matrix Id parsing error")]
    // MatrixIdParse(#[from] matrix_sdk_appservice::ruma::IdParseError),

    #[error("Login credential is invalid")]
    InvalidLoginCredential,
}

impl From<std::convert::Infallible> for Error {
    fn from(err: std::convert::Infallible) -> Error {
        match err { }
    }
}

impl<R: std::error::Error + 'static> From<ruma::client::Error<hyper::Error, R>> for Error {
    fn from(err: ruma::client::Error<hyper::Error, R>) -> Self {
        Self::RumaClient(Box::new(err))
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        match dbg!(err) {
            Error::Io(err) => err,
            err => std::io::Error::new(std::io::ErrorKind::Other, err.to_string()),
        }
    }
}
