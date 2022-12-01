use std::{sync::Arc, convert::Infallible};
use axum::{Form, http::request::Parts, extract::{FromRequestParts, State}, response::Redirect};
use axum_extra::extract::{PrivateCookieJar, cookie::{Key as CookieKey, Cookie}};
use east::{render, render_with_component};
use morum_ui::{App, Login, AnyComponent};
use async_trait::async_trait;
use serde::Deserialize;
use cookie::SameSite;
use super::{AppState, UserError, Html};

pub struct User {
    username: Option<String>,
    jar: PrivateCookieJar,
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<CookieKey>::from_request_parts(parts, state).await?;
        let username = jar.get("login").map(|cookie| cookie.value().to_owned());

        Ok(User {
            username,
            jar
        })
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

pub async fn view_login(user: User, State(context): State<AppState>) -> Result<Html, UserError> {
    if user.logged_in() {
        return Err(UserError::AlreadyLoggedIn)
    }

    Ok(Html {
        header: render! {
            title { "Login | morum" }
        },
        body: render_with_component!(AnyComponent, {
            App {
                logged_in: user.logged_in(),
                Login { }
            },
        }),
    })
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum LoginForm {
    Login {
        username: String,
        password: String,
    }
}

pub async fn act_login(user: User, State(context): State<AppState>, Form(form): Form<LoginForm>) -> Result<(PrivateCookieJar, Redirect), UserError> {
    match form {
        LoginForm::Login {
            username,
            password,
        } => {
            let valid = {
                let mut found = false;

                for user in &context.config.closed_beta_users {
                    if username == user.username && password == user.password {
                        found = true;
                        break;
                    }
                }

                found
            };

            if valid {
                let mut jar = user.into_jar();
                jar = jar.add(
                    Cookie::build("login", username)
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .finish()
                );

                Ok((jar, Redirect::to("/")))
            } else {
                Err(UserError::InvalidLoginCredential)
            }
        },
    }
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum LogoutForm {
    Logout { }
}

pub async fn act_logout(user: User, State(context): State<AppState>, Form(form): Form<LogoutForm>) -> Result<(PrivateCookieJar, Redirect), UserError> {
    match form {
        LogoutForm::Logout { } => {
            let mut jar = user.into_jar();
            jar = jar.remove(Cookie::named("login"));

            Ok((jar, Redirect::to("/")))
        },
    }
}
