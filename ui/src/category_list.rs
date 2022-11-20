use crate::{Persisted, Route, API_PREFIX};
use gloo_net::http::Request;
use morum_base::{params, types};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn CategoryList() -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let categories = use_state::<Option<Vec<types::Category>>, _>(|| None);

    if let Some(categories) = (*categories).clone() {
        html! {
            <>{
                categories.iter().map(|c| html! { <CategoryListCategory category={c.clone()} /> })
                    .collect::<Html>()
            }</>
        }
    } else {
        let categories = categories.clone();
        yew::platform::spawn_local(async move {
            let res = Request::get(&(API_PREFIX.to_owned() + "/api/native/categories"))
                .send()
                .await
                .unwrap()
                .json::<params::CategoriesResponse>()
                .await
                .unwrap();

            categories.set(Some(res.categories));
        });

        html! {
            <p>{"Loading ..."}</p>
        }
    }
}

#[derive(Properties, PartialEq)]
struct CategoryListCategoryProps {
    pub category: types::Category,
}

#[function_component]
fn CategoryListCategory(props: &CategoryListCategoryProps) -> Html {
    html! {
        <>
            <div class="row mb-1">
                <h4>{props.category.title.clone()}<small>{props.category.topic.clone()}</small></h4>
            </div>
            <div class="row mb-3">
                { props.category.subcategories.iter()
                      .map(|s| html! { <CategoryListSubcategory subcategory={s.clone()} /> }).collect::<Html>() }
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryListSubcategoryProps {
    pub subcategory: types::Subcategory,
}

#[function_component]
fn CategoryListSubcategory(props: &CategoryListSubcategoryProps) -> Html {
    html! {
        <div class="col-sm-6">
            <div class="card">
                <div class="card-body">
                    <h5 class="card-title"><Link<Route> to={Route::Posts { category_id: props.subcategory.id.clone().unwrap_or("uncategorized".to_string()) }}>{props.subcategory.title.clone()}</Link<Route>></h5>
                    <p class="card-text">{props.subcategory.topic.clone()}</p>
                </div>
            </div>
        </div>
    }
}
