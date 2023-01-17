use crate::AnyComponent;
use east::{render, render_with_component, Markup, Render};
use morum_base::types;

pub struct PostList {
    pub category: types::Category,
    pub posts: Vec<types::Post>,
}

impl Render<AnyComponent> for PostList {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "row mb-3",
                h3 {
                    self.category.title,
                    small { self.category.topic },
                },
            },

            self.posts.into_iter().map(|post| render! {
                div {
                    class: "row",
                    div {
                        class: "card",
                        h5 {
                            class: "card-title",
                            a {
                                href: format!("/post/{}", post.room_id),
                                post.title
                            },
                        },
                        post.topic.map(|topic| render! {
                            p { class: "card-text", topic }
                        }),
                        hr { },
                    },
                },
            }).collect::<Vec<_>>(),
        })
    }
}
