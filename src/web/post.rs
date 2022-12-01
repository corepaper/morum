use std::sync::Arc;
use serde::Deserialize;
use axum::{Form, response::{Redirect, IntoResponse}, extract::{State, Path}};
use east::{render, render_with_component};
use morum_base::types;
use morum_ui::{App, Post, AnyComponent};
use crate::Error;
use super::{AppState, UserError, User, Html};

pub async fn view_post(user: User, State(context): State<AppState>, Path(id): Path<usize>) -> Result<Html, UserError> {
    let rooms = context.appservice.valid_rooms().await?;
    let mut post = None;
    for room in rooms {
        if room.post_id == id {
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
    NewComment {
        comment: String,
    },
}

pub async fn act_post(user: User, State(context): State<AppState>, Path(id): Path<usize>, Form(form): Form<PostForm>) -> Result<Redirect, UserError> {
    match form {
        PostForm::NewComment {
            comment
        } => {
            if &comment == "" {
                return Err(Error::BlankContent.into());
            }

            let username = user.username().to_owned().ok_or(UserError::RequireLogin)?;

            let localpart = format!("forum_user_{}", username);

            let rooms = context.appservice.valid_rooms().await?;
            let mut post = None;
            for room in rooms {
                if room.post_id == id {
                    post = Some(types::Post {
                        title: room.title,
                        topic: room.topic,
                        id: room.post_id,
                    });
                }
            }
            let post = post.ok_or(Error::UnknownPost)?;

            let room_alias = format!("#forum_post_{}:corepaper.org", id);

            context
                .appservice
                .send_message(&localpart, &room_alias, &comment)
                .await?;

            Ok(Redirect::to(&format!("/post/{}", id)))
        },
    }
}
