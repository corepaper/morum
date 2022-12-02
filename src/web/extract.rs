use super::{AppState, Html};
use async_trait::async_trait;
use http::Request;
use axum::{
    BoxError,
    body::HttpBody,
    extract::{FromRequest, FromRequestParts},
    http::request::Parts,
    response::Redirect,
};
use axum_extra::extract::{
    cookie::{Cookie, Key as CookieKey},
    PrivateCookieJar,
};
use cookie::SameSite;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, Login};
use serde::{Deserialize, de::DeserializeOwned};
use std::{ops::Deref, convert::Infallible};
use crate::Error;

pub struct User {
    username: Option<String>,
    jar: PrivateCookieJar,
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<CookieKey>::from_request_parts(parts, state).await?;
        let username = jar.get("login").map(|cookie| cookie.value().to_owned());

        Ok(User { username, jar })
    }
}

impl User {
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn logged_in(&self) -> bool {
        self.username().is_some()
    }

    pub fn into_jar(self) -> PrivateCookieJar {
        self.jar
    }
}

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
