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

            div {
                class: "row",
                NewPost { },
            },
        })
    }
}

pub struct NewPost {}

impl Render<AnyComponent> for NewPost {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "col-12",
                form {
                    method: "post",
                    input {
                        type_: "hidden",
                        name: "action",
                        value: "NewPost",
                    },
                    div {
                        class: "form-group",
                        label { for_: "title", "Matrix Room ID" },
                        input { type_: "text", class: "form-control", id: "title", name: "room_id" },
                    },
                    input {
                        class: "btn btn-primary pull-right",
                        type_: "submit",
                        value: "Add a new post",
                    },
                }
            }
        })
    }
}
