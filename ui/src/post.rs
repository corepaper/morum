use crate::AnyComponent;
use east::{render, render_with_component, Markup, PreEscaped, Render};
use morum_base::types;

pub struct Post {
    pub post: types::Post,
    pub comments: Vec<types::Comment>,
}

impl Render<AnyComponent> for Post {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "row mb-3",
                h3 {
                    self.post.title,
                    br { },
                    self.post.topic.map(|t| {
                        render! { small { t } }
                    })
                },
            },

            self.comments.into_iter().map(|comment| {
                render! {
                    div {
                        class: "row",
                        div {
                            class: "card",
                            p {
                                class: "card-text",
                                strong { comment.sender }
                            },
                            p {
                                class: "card-text",
                                PreEscaped(comment.html),
                            }
                        },
                        hr { }
                    }
                }
            }).collect::<Vec<_>>(),

            div {
                class: "row",
                a {
                    "Post a new comment",
                    class: "btn btn-primary",
                    href: format!("https://matrix.to/#/{}", self.post.room_id),
                },
            },
        })
    }
}
