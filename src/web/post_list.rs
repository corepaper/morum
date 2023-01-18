use super::{extract, AppState, Html};
use crate::Error;
use axum::response::Redirect;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, PostList};
use serde::Deserialize;

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
            title { format!("{} | morum", category.title) },
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

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum PostListForm {
    NewPost { room_id: String },
}

pub async fn act_post_list(
    context: extract::State<AppState>,
    path: extract::Path<String>,
    form: extract::Form<PostListForm>,
) -> Result<Redirect, Error> {
    let category_room_local_id = path.0;

    match form.0 {
        PostListForm::NewPost { room_id } => {
            context
                .matrix
                .add_room_to_space(
                    format!("#forum-{}:corepaper.org", category_room_local_id),
                    room_id,
                )
                .await?;

            Ok(Redirect::to(&format!(
                "/category/{}",
                category_room_local_id
            )))
        }
    }
}
