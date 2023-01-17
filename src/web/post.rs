use super::{extract, AppState, Html};
use crate::Error;
use east::{render, render_with_component};
use morum_base::types;
use morum_ui::{AnyComponent, App, Post};

pub async fn view_post(
    context: extract::State<AppState>,
    path: extract::Path<usize>,
) -> Result<Html, Error> {
    let rooms = context.matrix.valid_rooms().await?;
    let mut post = None;
    for room in rooms {
        if room.post_id == path.0 {
            post = Some(types::Post {
                title: room.title,
                topic: room.topic,
                id: room.post_id,
            });
        }
    }
    let post = post.ok_or(Error::UnknownPost)?;

    let messages = context
        .matrix
        .messages(&format!("#forum_post_{}:corepaper.org", post.id))
        .await?;
    let mut comments = Vec::new();
    for message in messages {
        comments.push(types::Comment {
            html: message.html,
            sender: message.sender,
        });
    }

    Ok(Html {
        header: render! {
            title { "Post | morum" },
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
