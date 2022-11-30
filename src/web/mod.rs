mod user_error;

pub use self::user_error::UserError;

use std::net::SocketAddr;
use axum::{routing, Router};
use morum_ui::AnyComponent;
use east::{render, render_with_component};
use crate::Error;

east_build::include_trunk_assets! {
    Asset = Asset,
    Html = Html,
    TRUNK_ASSET_FILES = TRUNK_ASSET_FILES,
    route_trunk_assets = route_trunk_assets,
    replace_header = "<!-- header -->",
    replace_body = "<!-- body -->",
}

pub async fn start() -> Result<(), Error> {
    let mut app = Router::new();
    app = route_trunk_assets(app);

    app = app.route("/", routing::get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_world() -> Html {
    Html {
        header: render! {
            title { "Hello, world!" }
        },
        body: render_with_component!(AnyComponent, {
            p { "Hello, world!" }
        }),
    }
}
