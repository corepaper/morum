use std::sync::Arc;
use serde::Deserialize;
use axum::{Form, response::{Redirect, IntoResponse}, extract::{State, Path}};
use east::{render, render_with_component};
use morum_base::types;
use morum_ui::{App, PostList, AnyComponent};
use crate::Error;
use super::{AppState, UserError, User, Html};

pub async fn view_post_list(user: User, State(context): State<AppState>, Path(id): Path<String>) -> Result<Html, UserError> {
    let id = if id == "uncategorized" {
        None
    } else {
        Some(id)
    };

    let mut category = None;
    let mut subcategory = None;

    for c in &context.config.categories {
        for sc in &c.subcategories {
            if sc.id == id {
                category = Some(c.clone());
                subcategory = Some(sc.clone());
            }
        }
    }

    let category = category.ok_or(Error::UnknownCategory)?;
    let subcategory = subcategory.ok_or(Error::UnknownCategory)?;

    let rooms = context.appservice.valid_rooms().await?;
    let mut posts = Vec::new();

    for room in rooms {
        if room.category == id {
            posts.push(types::Post {
                title: room.title,
                topic: room.topic,
                id: room.post_id,
            });
        }
    }

    Ok(Html {
        header: render! {
            title { "Post List | morum" },
        },
        body: render_with_component!(AnyComponent, {
            App {
                logged_in: user.logged_in(),
                PostList {
                    category: category,
                    subcategory: subcategory,
                    posts: posts,
                },
            },
        }),
    })
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum PostListForm {
    NewPost {
        title: String,
        topic: String,
        markdown: String,
    },
}

pub async fn act_post_list(user: User, State(context): State<AppState>, Path(id): Path<String>, Form(form): Form<PostListForm>) -> Result<Redirect, UserError> {
    match form {
        PostListForm::NewPost {
            title, topic, markdown,
        } => {
            if &title == "" {
                return Err(Error::BlankTitle.into());
            }

            if &topic == "" {
                return Err(Error::BlankTopic.into());
            }

            if &markdown == "" {
                return Err(Error::BlankContent.into());
            }

            let category_id = if id == "uncategorized" {
                None
            } else {
                Some(id.clone())
            };
            let username = user.username().to_owned().ok_or(UserError::RequireLogin)?;

            let localpart = format!("forum_user_{}", username);

            let rooms = context.appservice.valid_rooms().await?;
            let room_id = rooms
                .iter()
                .map(|r| r.post_id)
                .reduce(|acc, r| if acc >= r { acc } else { r })
                .unwrap_or(1)
                + 1;

            let room_alias_localpart = format!("forum_post_{}", room_id);
            context
                .appservice
                .create_room(&room_alias_localpart, &title, &topic)
                .await?;

            let room_alias = format!("#forum_post_{}:corepaper.org", room_id);
            context
                .appservice
                .set_category(&room_alias, category_id)
                .await?;

            context
                .appservice
                .send_message(&localpart, &room_alias, &markdown)
                .await?;

            Ok(Redirect::to(&format!("/category/{}", id)))
        },
    }
}