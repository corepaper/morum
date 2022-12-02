use crate::Error;
use http::Request;
use axum::{
    TypedHeader,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    middleware::Next,
    extract::{State, Query},
    headers,
};
use thiserror::Error;
use east::{render, render_with_component};
use morum_ui::{App, AnyComponent};
use std::collections::HashMap;
use super::{User, Html, AppState};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Internal error")]
    Internal,
    #[error("Login credential is invalid")]
    InvalidLoginCredential,
    #[error("Already logged in")]
    AlreadyLoggedIn,
    #[error("Require login")]
    RequireLogin,
}

impl UserError {
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::Internal => Some(StatusCode::INTERNAL_SERVER_ERROR),
            Self::InvalidLoginCredential => Some(StatusCode::UNAUTHORIZED),
            Self::AlreadyLoggedIn => None,
            Self::RequireLogin => None,
        }
    }

    pub fn redirect(&self) -> Option<Redirect> {
        match self {
            Self::AlreadyLoggedIn => Some(Redirect::to("/")),
            Self::RequireLogin => Some(Redirect::to("/login")),
            _ => None,
        }
    }
}

impl From<Error> for UserError {
    fn from(err: Error) -> Self {
        match err {
            Error::AlreadyLoggedIn => Self::AlreadyLoggedIn,
            Error::InvalidLoginCredential => Self::InvalidLoginCredential,
            Error::RequireLogin => Self::RequireLogin,

            _ => Self::Internal,
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let mut res = Response::new(Default::default());
        res.extensions_mut().insert(self);
        res
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let user_error = UserError::from(self);
        user_error.into_response()
    }
}

pub async fn handle_error<B>(user: User, req: Request<B>, next: Next<B>) -> Response {
    let host = dbg!(req.headers().get("Host"));
    let referer = dbg!(req.headers().get("Referer"));
    let go_back = host
        .and_then(|host| host.to_str().ok())
        .and_then(|host| referer.map(|referer| (host, referer)))
        .and_then(|(host, referer)| referer.to_str().ok().map(|referer| (host, referer)))
        .map(|(host, referer)| (host.to_string(), referer.to_string()))
        .and_then(|(host, referer)| {
            if referer.starts_with(&format!("https://{}", host)) || (
                referer.starts_with(&format!("http://{}", host)) && host.starts_with("127.0.0.1:")) {
                Some(referer)
            } else {
                None
            }
        });


    let res = next.run(req).await;

    if let Some(error) = res.extensions().get::<UserError>() {
        if let Some(redirect) = error.redirect() {
            return redirect.into_response()
        }

        let html = Html {
            header: render! {
                title { "Error | morum" },
            },
            body: render_with_component!(AnyComponent, {
                App {
                    logged_in: user.logged_in(),
                    p {
                        class: "error",
                        format!("{:?}", error),

                        br { },

                        dbg!(go_back).map(|go_back| {
                            render! {
                                a {
                                    class: "btn btn-primary",
                                    href: go_back.clone(),
                                    "Go back",
                                }
                            }
                        }),
                    },
                },
            }),
        };

        if let Some(status_code) = error.status_code() {
            return (status_code, html).into_response()
        } else {
            return html.into_response()
        }
    }

    res
}
