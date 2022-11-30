mod category_list;
mod login;
mod post;
mod post_list;

use serde::{Serialize, Deserialize};
use east::{HydrateTo, Render, RenderMulti, Markup, render_with_component, render_from_multi};

#[derive(Serialize, Deserialize, HydrateTo, Debug, Clone)]
pub enum AnyComponent { }

pub struct App {
    pub logged_in: bool,
}

#[render_from_multi]
impl RenderMulti<AnyComponent> for App {
    fn render_multi(self, children: Markup) -> Markup {
        render_with_component!(AnyComponent, {
            Nav { logged_in: self.logged_in },
            div {
                class: "container m-3",
                children,
            },
            Footer { },
        })
    }
}

pub struct Nav {
    pub logged_in: bool
}

impl Render<AnyComponent> for Nav {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            nav {
                class: "navbar navbar-light navbar-expand-sm",
                span {
                    class: "navbar-brand",
                    a { href: "/", "morum" }
                },
                ul { class: "navbar-bav" },
                NavLoginLink { logged_in: self.logged_in }
            }
        })
    }
}

pub struct Footer { }

impl Render<AnyComponent> for Footer {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            footer {
                class: "text-center text-lg-start text-muted mt-3",
                a { href: "https://that.world/legal.txt", target: "_blank", "Legal notice" }, ". ",
                "Copyright (c) 2022 Wei Tang. morum is licensed under ",
                a { href: "https://github.com/corepaper/morum", target: "_blank", "AGPL-3.0" }, ". ",
            }
        })
    }
}

pub struct NavLoginLink {
    pub logged_in: bool,
}

impl Render<AnyComponent> for NavLoginLink {
    fn render(self) -> Markup {
        if self.logged_in {
            render_with_component!(AnyComponent, {
                div {
                    class: "login",
                    span {
                        class: "navbar-text",
                        form {
                            action: "/logout", method: "post",
                            input { type_: "submit", value: "Logout" }
                        }
                    }
                }
            })
        } else {
            render_with_component!(AnyComponent, {
                div {
                    class: "login",
                    span {
                        class: "navbar-text",
                        a { href: "/login", "Login" }
                    }
                }
            })
        }
    }
}
