use crate::{Persisted, Route, API_PREFIX};
use gloo_net::http::Request;
use morum_base::{params, types};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PostListProps {
    pub category_id: String,
}

#[function_component]
pub fn PostList(props: &PostListProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let fetched =
        use_state::<Option<(types::Category, types::Subcategory, Vec<types::Post>)>, _>(|| None);

    if let Some((category, subcategory, posts)) = (*fetched).clone() {
        let reset = {
            let fetched = fetched.clone();

            Callback::from(move |_| fetched.set(None))
        };

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
                            <h5 class="card-title"><Link<Route> to={Route::Post { post_id: post.id }}>{post.title.clone()}</Link<Route>></h5>
                            if let Some(topic) = post.topic.clone() {
                                <p class="card-text">{topic}</p>
                            }
                            <hr />
                        </div>
                    </div>
                }).collect::<Html>() }

                <div class="row">
                    <NewPost category_id={props.category_id.clone()} reset={reset} />
                </div>
            </>
        }
    } else {
        let fetched = fetched.clone();
        let props = props.clone();

        yew::platform::spawn_local(async move {
            let res = Request::get(
                &(API_PREFIX.to_owned()
                    + &format!("/api/native/posts?category_id={}", props.category_id)),
            )
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

#[derive(Properties, PartialEq, Clone)]
pub struct NewPostProps {
    pub category_id: String,
    pub reset: Callback<()>,
}

#[function_component]
pub fn NewPost(props: &NewPostProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let disabled = (*persisted).access_token.is_none();

    let title = use_state(|| "".to_owned());
    let ontitle = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let topic = use_state(|| "".to_owned());
    let ontopic = {
        let topic = topic.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            topic.set(input.value());
        })
    };

    let markdown = use_state(|| "".to_owned());
    let onmarkdown = {
        let markdown = markdown.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            markdown.set(input.value());
        })
    };

    let onclick = {
        let persisted = persisted.clone();
        let title = title.clone();
        let topic = topic.clone();
        let markdown = markdown.clone();
        let reset = props.reset.clone();
        let category_id = props.category_id.clone();

        Callback::from(move |_| {
            let persisted = persisted.clone();
            let title = title.clone();
            let topic = topic.clone();
            let markdown = markdown.clone();
            let reset = reset.clone();
            let category_id = category_id.clone();

            yew::platform::spawn_local(async move {
                let res = Request::post(&(API_PREFIX.to_owned() + "/api/native/new_post"))
                    .json(&params::NewPost {
                        access_token: (*persisted).access_token.clone().unwrap(),
                        title: (*title).clone(),
                        topic: (*topic).clone(),
                        markdown: (*markdown).clone(),
                        category_id: if &category_id == "uncategorized" {
                            None
                        } else {
                            Some(category_id.clone())
                        },
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<params::NewPostResponse>()
                    .await
                    .unwrap();

                reset.emit(());
            });
        })
    };

    html! {
        <div class="col-12">
            <div class="form-group">
                <label for="title">{"Title"}</label>
                <input type="text" class="form-control" id="title" oninput={ontitle} disabled={disabled} />
            </div>
            <div class="form-group">
                <label for="topic">{"Topic"}</label>
                <input type="text" class="form-control" id="topic" oninput={ontopic} disabled={disabled} />
            </div>
            <div class="form-group">
                <label for="content">{"Content"}</label>
                <textarea class="form-control" id="content" rows="5" oninput={onmarkdown} disabled={disabled}></textarea>
            </div>
            <button type="button" class="btn btn-primary pull-right" onclick={onclick} disabled={disabled}>{"Submit"}</button>
        </div>
    }
}
