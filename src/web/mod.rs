mod user_error;
mod category_list;
mod login;
mod post_list;
mod post;

pub use self::user_error::UserError;
pub use self::login::User;

use std::{sync::Arc, net::SocketAddr, ops::Deref};
use axum::{routing, Router, extract::FromRef};
use axum_extra::extract::cookie::Key as CookieKey;
use morum_ui::AnyComponent;
use east::{render, render_with_component};
use crate::{Config, AppService, Error};

pub struct Context {
    pub config: Config,
    pub appservice: AppService,
    pub cookie_key: CookieKey,
}

#[derive(Clone)]
pub struct AppState(Arc<Context>);

impl Deref for AppState {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl FromRef<AppState> for CookieKey {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

east_build::include_trunk_assets! {
    Asset = Asset,
    Html = Html,
    TRUNK_ASSET_FILES = TRUNK_ASSET_FILES,
    route_trunk_assets = route_trunk_assets,
    replace_header = "<!-- header -->",
    replace_body = "<!-- body -->",
}

pub async fn start(config: Config, appservice: AppService) -> Result<(), Error> {
    let context = Arc::new(Context {
        config,
        appservice,
        cookie_key: CookieKey::generate(),
    });

    let mut app: Router<AppState> = Router::new();

    app = route_trunk_assets(app);
    app = app
        .route("/", routing::get(self::category_list::view_category_list))
        .route("/login", routing::get(self::login::view_login).post(self::login::act_login))
        .route("/logout", routing::post(self::login::act_logout))
        .route("/category/:id", routing::get(self::post_list::view_post_list).post(self::post_list::act_post_list))
        .route("/post/:id", routing::get(self::post::view_post).post(self::post::act_post));

    let app: Router<()> = app.with_state(AppState(context));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
