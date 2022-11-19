use gloo_net::http::Request;
use morum_base::{params, types};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Persisted, API_PREFIX};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component]
pub fn RawHtml(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}

#[derive(Properties, PartialEq, Clone)]
pub struct PostProps {
    pub id: String,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let fetched = use_state::<Option<(types::Post, Vec<types::Comment>)>, _>(|| None);

    if let Some((post, comments)) = (*fetched).clone() {
        html! {
            <>
                <div class="row mb-3">
                    <h3>
                        {post.title.clone()}<br />
                        if let Some(topic) = post.topic.clone() {
                            <small>{topic}</small>
                        }
                    </h3>
                </div>

                { comments.iter().map(|comment| html! {
                    <div class="row">
                        <div class="card">
                            <p class="card-text"><strong>{ comment.sender.clone() }</strong></p>
                            <p class="card-text"><RawHtml html={comment.html.clone()} /></p>
                        </div>
                        <hr />
                    </div>
                }).collect::<Html>() }
            </>
        }
    } else {
        let fetched = fetched.clone();
        let props = props.clone();

        yew::platform::spawn_local(async move {
            let res = Request::get(&(API_PREFIX.to_owned() + &format!("/api/native/post?id={}", props.id)))
                .send()
                .await
                .unwrap()
                .json::<params::PostResponse>()
                .await
                .unwrap();

            fetched.set(Some((res.post, res.comments)));
        });

        html! {
            <p>{"Loading ..."}</p>
        }
    }
}
