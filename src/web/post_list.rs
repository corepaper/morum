use super::{extract, AppState, Html};
use crate::Error;
use east::{render, render_with_component};
use morum_base::types;
use morum_ui::{AnyComponent, App, PostList};

pub async fn view_post_list(
    context: extract::State<AppState>,
    path: extract::Path<String>,
) -> Result<Html, Error> {
    let id = if path.0 == "uncategorized" {
        None
    } else {
        Some(path.0)
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

    let rooms = context.matrix.valid_rooms().await?;
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
                PostList {
                    category: category,
                    subcategory: subcategory,
                    posts: posts,
                },
            },
        }),
    })
}
