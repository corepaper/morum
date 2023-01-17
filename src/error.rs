use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Json Web Token related errors")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Yaml parse error")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Json parse error")]
    Json(#[from] serde_json::Error),
    // #[error("Appservice error")]
    // Appservice(#[from] matrix_sdk_appservice::Error),
    #[error("Url parsing error")]
    UrlParse(#[from] url::ParseError),
    #[error("Ruma client error")]
    RumaClient(Box<dyn std::error::Error>),
    #[error("Matrix client build error")]
    MatrixClientBuild(#[from] matrix_sdk::ClientBuildError),
    #[error("Matrix error")]
    Matrix(#[from] matrix_sdk::Error),
    #[error("Matrix HTTP error")]
    MatrixHttp(#[from] matrix_sdk::HttpError),
    #[error("Matrix Id parsing error")]
    MatrixIdParse(#[from] ruma::IdParseError),
    #[error("System time error")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("Hyper error")]
    Hyper(#[from] hyper::Error),
    #[error("Axum form error")]
    AxumForm(#[from] axum::extract::rejection::FormRejection),

    #[error("Already logged in")]
    AlreadyLoggedIn,
    #[error("Require logging in")]
    RequireLogin,
    #[error("Login credential is invalid")]
    InvalidLoginCredential,
    #[error("Unknown category")]
    UnknownCategory,
    #[error("Unknown post")]
    UnknownPost,
    #[error("Unknown post title")]
    UnknownPostTitle,
    #[error("Unknown category title")]
    UnknownCategoryTitle,
    #[error("Unknown category topic")]
    UnknownCategoryTopic,
    #[error("Unknown toplevel room")]
    UnknownToplevelRoom,
    #[error("Unknown category room")]
    UnknownCategoryRoom,
    #[error("Unknown category room")]
    InvalidCategoryAlias,
}

impl From<std::convert::Infallible> for Error {
    fn from(err: std::convert::Infallible) -> Error {
        match err {}
    }
}

impl<R: std::error::Error + 'static> From<ruma::client::Error<matrix_sdk::reqwest::Error, R>>
    for Error
{
    fn from(err: ruma::client::Error<matrix_sdk::reqwest::Error, R>) -> Self {
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
