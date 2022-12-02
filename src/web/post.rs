use super::{extract, AppState, Html};
use crate::Error;
use axum::{
    response::Redirect,
    Form,
};
use east::{render, render_with_component};
use morum_base::types;
use morum_ui::{AnyComponent, App, Post};
use serde::Deserialize;

pub async fn view_post(
    user: extract::User,
    context: extract::State<AppState>,
    path: extract::Path<usize>,
) -> Result<Html, Error> {
    let rooms = context.appservice.valid_rooms().await?;
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
        .appservice
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
                logged_in: user.logged_in(),
                Post {
                    post: post,
                    comments: comments,
                },
            },
        }),
    })
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum PostForm {
    NewComment { comment: String },
}

pub async fn act_post(
    user: extract::User,
    context: extract::State<AppState>,
    path: extract::Path<usize>,
    form: extract::Form<PostForm>,
) -> Result<Redirect, Error> {
    match form.0 {
        PostForm::NewComment { comment } => {
            if &comment == "" {
                return Err(Error::BlankContent.into());
            }

            let username = user.username().to_owned().ok_or(Error::RequireLogin)?;

            let localpart = format!("forum_user_{}", username);

            let rooms = context.appservice.valid_rooms().await?;
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
            let _post = post.ok_or(Error::UnknownPost)?;

            let room_alias = format!("#forum_post_{}:corepaper.org", path.0);

            context
                .appservice
                .send_message(&localpart, &room_alias, &comment)
                .await?;

            Ok(Redirect::to(&format!("/post/{}", path.0)))
        }
    }
}
