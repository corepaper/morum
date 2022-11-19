use gloo_net::http::Request;
use morum_base::{params, types};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Persisted, API_PREFIX};

#[derive(Properties, PartialEq, Clone)]
pub struct PostListProps {
    pub category_id: String,
}

#[function_component]
pub fn PostList(props: &PostListProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let fetched = use_state::<Option<(types::Category, types::Subcategory, Vec<types::Post>)>, _>(|| None);

    if let Some((category, subcategory, posts)) = (*fetched).clone() {
        html! {
            <>
                <div class="row mb-3">
                    <h3>
                        {category.title.clone()}{" > "}{subcategory.title.clone()}
                        <small>{subcategory.topic.clone()}</small>
                    </h3>
                </div>
                { posts.iter().map(|post| html! {
                    <div class="row">
                        <div class="card">
                            <h5 class="card-title"><Link<Route> to={Route::Post { post_id: format!("{}", post.id) }}>{post.title.clone()}</Link<Route>></h5>
                            if let Some(topic) = post.topic.clone() {
                                <p class="card-text">{topic}</p>
                            }
                            <hr />
                        </div>
                    </div>
                }).collect::<Html>() }
            </>
        }
    } else {
        let fetched = fetched.clone();
        let props = props.clone();

        yew::platform::spawn_local(async move {
            let res = Request::get(&(API_PREFIX.to_owned() + &format!("/api/native/posts?category_id={}", props.category_id)))
                .send()
                .await
                .unwrap()
                .json::<params::PostsResponse>()
                .await
                .unwrap();

            fetched.set(Some((res.category, res.subcategory, res.posts)));
        });

        html! {
            <p>{"Loading ..."}</p>
        }
    }
}
