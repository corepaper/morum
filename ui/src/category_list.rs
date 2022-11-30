use morum_base::types;
use east::{Render, Markup, render_with_component};
use crate::AnyComponent;

pub struct CategoryList {
    pub categories: Vec<types::Category>,
}

impl Render<AnyComponent> for CategoryList {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            self.categories.into_iter().map(|c| {
                CategoryListCategory { category: c }
            }).collect::<Vec<_>>()
        })
    }
}

pub struct CategoryListCategory {
    pub category: types::Category,
}

impl Render<AnyComponent> for CategoryListCategory {
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            div {
                class: "row mb-1",
                h4 {
                    self.category.title,
                    small { self.category.topic },
                }
            },
            div {
                class: "row mb-3",
                self.category.subcategories.into_iter().map(|s| {
                    CategoryListSubcategory { subcategory: s }
                }).collect::<Vec<_>>()
            }
        })
    }
}

pub struct CategoryListSubcategory {
    pub subcategory: types::Subcategory,
}

impl Render<AnyComponent> for CategoryListSubcategory {
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
                                href: format!("/category/{}", self.subcategory.id.unwrap_or("uncategorized".to_string())),
                                self.subcategory.title,
                            }
                        },
                        p { class: "card-text", self.subcategory.topic },
                    },
                },
            }
        })
    }
}
