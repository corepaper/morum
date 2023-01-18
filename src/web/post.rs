use super::{extract, AppState, Html};
use crate::Error;
use east::{render, render_with_component};
use morum_ui::{AnyComponent, App, Post};

pub async fn view_post(
    context: extract::State<AppState>,
    path: extract::Path<String>,
) -> Result<Html, Error> {
    let room_id = path.0;

    let (post, comments) = context.matrix.post_comments(room_id).await?;

    Ok(Html {
        header: render! {
            title { format!("{} | morum", post.title) },
        },
        body: render_with_component!(AnyComponent, {
            App {
                Post {
                    post: post,
                    comments: comments,
                },
            },
        }),
    })
}
