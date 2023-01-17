use super::{extract, AppState, Html};
use crate::Error;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, CategoryList};

pub async fn view_category_list(context: extract::State<AppState>) -> Result<Html, Error> {
    let categories = context.matrix.categories().await?;

    Ok(Html {
        header: render! {
            title { "Home | morum" },
        },
        body: render_with_component!(AnyComponent, {
            App {
                CategoryList {
                    categories: categories,
                },
            },
        }),
    })
}
