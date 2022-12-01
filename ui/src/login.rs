use east::{Render, Markup, render_with_component};
use crate::AnyComponent;

pub struct Login { }

impl Render<AnyComponent> for Login {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            form {
                action: "/login",
                method: "post",

                input {
                    type_: "hidden",
                    name: "action",
                    value: "Login",
                },

                div {
                    class: "row",
                    div {
                        class: "col-md-10 offset-md-1",
                        h3 { "Log in to morum" }
                    }
                },

                div {
                    class: "row",
                    div {
                        class: "col-md-6 offset-md-3",
                        div {
                            class: "form-group",
                            label { "Username" },
                            input { class: "form-control", type_: "text", name: "username" }
                        },
                        div {
                            class: "form-group",
                            label { "Password" },
                            input { class: "form-control", type_: "password", name: "password" }
                        },
                        input {
                            class: "btn btn-primary pull-right",
                            type_: "submit",
                            value: "Submit"
                        },
                    }
                },
            }
        })
    }
}
