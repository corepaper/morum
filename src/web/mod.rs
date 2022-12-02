mod category_list;
mod login;
mod post;
mod post_list;
mod user_error;
mod extract;

pub use self::user_error::UserError;

use crate::{AppService, Config, Error};
use axum::{extract::FromRef, middleware, routing, Router};
use axum_extra::extract::cookie::Key as CookieKey;
use std::{net::SocketAddr, ops::Deref, sync::Arc};

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
        .route(
            "/login",
            routing::get(self::login::view_login).post(self::login::act_login),
        )
        .route("/logout", routing::post(self::login::act_logout))
        .route(
            "/category/:id",
            routing::get(self::post_list::view_post_list).post(self::post_list::act_post_list),
        )
        .route(
            "/post/:id",
            routing::get(self::post::view_post).post(self::post::act_post),
        );

    let state = AppState(context);

    let app: Router<()> = app
        .layer(middleware::from_fn_with_state(state.clone(), self::user_error::handle_error))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
