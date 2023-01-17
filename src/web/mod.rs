mod category_list;
mod extract;
mod post;
mod post_list;
mod user_error;

pub use self::user_error::UserError;

use crate::{Config, Error, MatrixService};
use axum::{middleware, routing, Router};
use std::{net::SocketAddr, ops::Deref, sync::Arc};

pub struct Context {
    pub config: Config,
    pub matrix: MatrixService,
}

#[derive(Clone)]
pub struct AppState(Arc<Context>);

impl Deref for AppState {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &*self.0
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

pub async fn start(config: Config, matrix: MatrixService) -> Result<(), Error> {
    let context = Arc::new(Context { config, matrix });

    let mut app: Router<AppState> = Router::new();

    app = route_trunk_assets(app);
    app = app
        .route("/", routing::get(self::category_list::view_category_list))
        .route(
            "/category/:id",
            routing::get(self::post_list::view_post_list).post(self::post_list::act_post_list),
        )
        .route("/post/:id", routing::get(self::post::view_post));

    let state = AppState(context);

    let app: Router<()> = app
        .layer(middleware::from_fn_with_state(
            state.clone(),
            self::user_error::handle_error,
        ))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
