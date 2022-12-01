use east::{Render, Markup, PreEscaped, render, render_with_component};
use morum_base::types;
use crate::AnyComponent;

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
                NewComment { }
            }
        })
    }
}

pub struct NewComment { }

impl Render<AnyComponent> for NewComment {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "col-12",
                form {
                    method: "post",
                    input {
                        type_: "hidden",
                        name: "action",
                        value: "NewComment",
                    },
                    div {
                        class: "form-group",
                        label { for_: "new-comment", "New comment" },
                        textarea { class: "form-control", id: "new-comment", name: "comment", rows: "5", },
                    },
                    input {
                        class: "btn btn-primary pull-right",
                        type_: "submit",
                        value: "Submit",
                    }
                }
            }
        })
    }
}
