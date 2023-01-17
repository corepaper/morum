use crate::AnyComponent;
use east::{render_with_component, Markup, Render};
use morum_base::types;

pub struct CategoryList {
    pub categories: Vec<types::Category>,
}

impl Render<AnyComponent> for CategoryList {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "row mb-1",
                h4 {
                    "Categories",
                    small { "All categories" },
                }
            },
            div {
                class: "row mb-3",
                self.categories.into_iter().map(|s| {
                    CategoryListItem { category: s }
                }).collect::<Vec<_>>()
            }
        })
    }
}

pub struct CategoryListItem {
    pub category: types::Category,
}

impl Render<AnyComponent> for CategoryListItem {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "col-sm-6",
                div {
                    class: "card",
                    div {
                        class: "card-body",
                        h5 {
                            class: "card-title",
                            a {
                                href: format!("/category/{}", self.category.room_local_id),
                                self.category.title,
                            }
                        },
                        p { class: "card-text", self.category.topic },
                    },
                },
            }
        })
    }
}
