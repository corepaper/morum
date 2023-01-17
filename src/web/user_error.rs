use super::Html;
use crate::Error;
use axum::{
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use east::{render, render_with_component};
use http::Request;
use morum_ui::{AnyComponent, App};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Internal error")]
    Internal,
}

impl UserError {
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::Internal => Some(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub fn redirect(&self) -> Option<Redirect> {
        match self {
            _ => None,
        }
    }
}

impl From<Error> for UserError {
    fn from(err: Error) -> Self {
        match err {
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

pub async fn handle_error<B>(req: Request<B>, next: Next<B>) -> Response {
    let host = req.headers().get("Host");
    let referer = req.headers().get("Referer");
    let go_back = host
        .and_then(|host| host.to_str().ok())
        .and_then(|host| referer.map(|referer| (host, referer)))
        .and_then(|(host, referer)| referer.to_str().ok().map(|referer| (host, referer)))
        .map(|(host, referer)| (host.to_string(), referer.to_string()))
        .and_then(|(host, referer)| {
            if referer.starts_with(&format!("https://{}", host))
                || (referer.starts_with(&format!("http://{}", host))
                    && host.starts_with("127.0.0.1:"))
            {
                Some(referer)
            } else {
                None
            }
        });

    let res = next.run(req).await;

    if let Some(error) = res.extensions().get::<UserError>() {
        if let Some(redirect) = error.redirect() {
            return redirect.into_response();
        }

        let html = Html {
            header: render! {
                title { "Error | morum" },
            },
            body: render_with_component!(AnyComponent, {
                App {
                    p {
                        class: "error",

                        match error {
                            UserError::Internal => "Internal error occured. Please try again later."
                        },

                        br { },

                        go_back.map(|go_back| {
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
            return (status_code, html).into_response();
        } else {
            return html.into_response();
        }
    }

    res
}
