use actix_web::{
    body::BoxBody,
    web::{self, Data},
    App, HttpResponse, HttpResponseBuilder, HttpServer,
};
use morum::{action::*, Error, UserError};
use morum_base::params::*;
use serde::Deserialize;
use std::sync::Arc;

static UI_FILES: include_dir::Dir<'static> = include_dir::include_dir!("$OUT_DIR/dist");

async fn perform<Request>(
    data: Request,
    context: web::Data<Arc<Context>>,
) -> Result<HttpResponse, UserError>
where
    Request: Perform,
    Request: Send + 'static,
{
    let res = data
        .perform(&context)
        .await
        .map(|json| HttpResponse::Ok().json(json))?;
    Ok(res)
}

async fn route_get<'a, Data>(
    data: web::Query<Data>,
    context: web::Data<Arc<Context>>,
) -> Result<HttpResponse, UserError>
where
    Data: Deserialize<'a> + Send + 'static + Perform,
{
    perform::<Data>(data.0, context).await
}

async fn route_post<'a, Data>(
    data: web::Json<Data>,
    context: web::Data<Arc<Context>>,
) -> Result<HttpResponse, UserError>
where
    Data: Deserialize<'a> + Send + 'static + Perform,
{
    perform::<Data>(data.0, context).await
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let context = Arc::new(Context::dev());

    HttpServer::new(move || {
        let mut app = App::new().wrap(actix_cors::Cors::permissive());

        app = app.service(
            web::scope("/api/native")
                .app_data(Data::new(context.clone()))
                .route("/user/login", web::post().to(route_post::<Login>)),
        );

        for entry in UI_FILES.entries() {
            if let Some(file) = entry.as_file() {
                let ext = file.path().extension().and_then(|s| s.to_str());
                let path = file.path().to_string_lossy();
                let content = file.contents().to_vec();

                if path == "index.html" {
                    app = app.default_service(web::get().to(move || {
                        let content = content.clone();

                        async move {
                            let mut res = HttpResponse::Ok();
                            res.content_type("text/html");

                            res.body(content.clone())
                        }
                    }));
                } else {
                    app = app.route(
                        &path,
                        web::get().to(move || {
                            let ext = ext.clone();
                            let content = content.clone();

                            async move {
                                let mut res = HttpResponse::Ok();

                                match ext {
                                    Some("html") => {
                                        res.content_type("text/html");
                                    }
                                    Some("css") => {
                                        res.content_type("text/css");
                                    }
                                    Some("js") => {
                                        res.content_type("text/javascript");
                                    }
                                    Some("wasm") => {
                                        res.content_type("application/wasm");
                                    }
                                    _ => (),
                                }

                                res.body(content.clone())
                            }
                        }),
                    );
                }
            }
        }

        app
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
