use crate::Error;
use async_trait::async_trait;
use axum::{body::HttpBody, extract::FromRequest, BoxError};
use http::Request;
use serde::de::DeserializeOwned;
use std::ops::Deref;

pub use axum::extract::State;

pub struct Form<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Form<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Form(axum::Form::from_request(req, state).await?.0))
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub use axum::extract::Path;
