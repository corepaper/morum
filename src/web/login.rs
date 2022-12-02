use super::{extract, AppState, Html};
use async_trait::async_trait;
use axum::{
    http::request::Parts,
    response::Redirect,
    Form,
};
use axum_extra::extract::{
    cookie::{Cookie, Key as CookieKey},
    PrivateCookieJar,
};
use cookie::SameSite;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, Login};
use serde::Deserialize;
use std::convert::Infallible;
use crate::Error;

pub async fn view_login(user: extract::User) -> Result<Html, Error> {
    if user.logged_in() {
        return Err(Error::AlreadyLoggedIn);
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
    Login { username: String, password: String },
}

pub async fn act_login(
    user: extract::User,
    context: extract::State<AppState>,
    form: extract::Form<LoginForm>,
) -> Result<(PrivateCookieJar, Redirect), Error> {
    match form.0 {
        LoginForm::Login { username, password } => {
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
                        .finish(),
                );

                Ok((jar, Redirect::to("/")))
            } else {
                Err(Error::InvalidLoginCredential)
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum LogoutForm {
    Logout {},
}

pub async fn act_logout(
    user: extract::User,
    form: extract::Form<LogoutForm>,
) -> Result<(PrivateCookieJar, Redirect), Error> {
    match form.0 {
        LogoutForm::Logout {} => {
            let mut jar = user.into_jar();
            jar = jar.remove(Cookie::named("login"));

            Ok((jar, Redirect::to("/")))
        }
    }
}
