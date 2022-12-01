use std::sync::Arc;
use axum::extract::State;
use east::{render, render_with_component};
use morum_ui::{App, CategoryList, AnyComponent};
use super::{Context, UserError, Html};

pub async fn category_list(State(context): State<Arc<Context>>) -> Result<Html, UserError> {
    let categories = context.config.categories.clone();

    Ok(Html {
        header: render! {
            title { "Home | morum" },
        },
        body: render_with_component!(AnyComponent, {
            App {
                logged_in: false,
                CategoryList {
                    categories: categories,
                },
            },
        }),
    })
}
