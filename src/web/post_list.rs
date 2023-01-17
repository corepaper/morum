use super::{extract, AppState, Html};
use crate::Error;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, PostList};

pub async fn view_post_list(
    context: extract::State<AppState>,
    path: extract::Path<String>,
) -> Result<Html, Error> {
    let room_local_id = path.0;

    let (category, posts) = context
        .matrix
        .category_posts(format!("#forum-{}:corepaper.org", room_local_id))
        .await?;

    Ok(Html {
        header: render! {
            title { "Post List | morum" },
        },
        body: render_with_component!(AnyComponent, {
            App {
                PostList {
                    category: category,
                    posts: posts,
                },
            },
        }),
    })
}
