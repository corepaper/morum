use east::{Render, Markup, render, render_with_component};
use morum_base::types;
use crate::AnyComponent;

pub struct PostList {
    pub category: types::Category,
    pub subcategory: types::Subcategory,
    pub posts: Vec<types::Post>,
}

impl Render<AnyComponent> for PostList {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "row mb-3",
                h3 {
                    self.category.title, " > ", self.subcategory.title,
                    small { self.subcategory.topic },
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
                                href: format!("/post/{}", post.id),
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
            }
        })
    }
}

pub struct NewPost { }

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
                        label { for_: "title", "Title" },
                        input { type_: "text", class: "form-control", id: "title", name: "title" },
                    },
                    div {
                        class: "form-group",
                        label { for_: "topic", "Topic" },
                        input { type_: "text", class: "form-control", id: "topic", name: "topic" },
                    },
                    div {
                        class: "form-group",
                        label { for_: "content", "Content" },
                        textarea { class: "form-control", id: "content", name: "markdown", rows: "5" },
                    },
                    input {
                        class: "btn btn-primary pull-right",
                        type_: "submit",
                        value: "Submit",
                    },
                }
            }
        })
    }
}
