mod user_error;
mod category_list;

pub use self::user_error::UserError;

use std::{sync::Arc, net::SocketAddr};
use axum::{routing, Router};
use morum_ui::AnyComponent;
use east::{render, render_with_component};
use crate::{Config, AppService, Error};

pub struct Context {
    pub config: Config,
    pub appservice: AppService,
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
    let context = Arc::new(Context { config, appservice });

    let mut app: Router<Arc<Context>> = Router::new();

    app = route_trunk_assets(app);
    app = app.route("/", routing::get(self::category_list::category_list));

    let app: Router<()> = app.with_state(context);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
