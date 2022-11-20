use crate::{Persisted, Route, API_PREFIX};
use gloo_net::http::Request;
use morum_base::{params, types};
use web_sys::HtmlTextAreaElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

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
    pub id: usize,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let fetched = use_state::<Option<(types::Post, Vec<types::Comment>)>, _>(|| None);

    if let Some((post, comments)) = (*fetched).clone() {
        let reset = {
            let fetched = fetched.clone();

            Callback::from(move |_| fetched.set(None))
        };

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

                <div class="row">
                    <NewComment id={props.id.clone()} reset={reset} />
                </div>
            </>
        }
    } else {
        let fetched = fetched.clone();
        let props = props.clone();

        yew::platform::spawn_local(async move {
            let res = Request::get(
                &(API_PREFIX.to_owned() + &format!("/api/native/post?id={}", props.id)),
            )
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

#[derive(Properties, PartialEq, Clone)]
pub struct NewCommentProps {
    pub id: usize,
    pub reset: Callback<()>,
}

#[function_component]
pub fn NewComment(props: &NewCommentProps) -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let disabled = (*persisted).access_token.is_none();

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
        let markdown = markdown.clone();
        let reset = props.reset.clone();
        let post_id = props.id.clone();

        Callback::from(move |_| {
            let persisted = persisted.clone();
            let markdown = markdown.clone();
            let reset = reset.clone();

            yew::platform::spawn_local(async move {
                let res = Request::post(&(API_PREFIX.to_owned() + "/api/native/new_comment"))
                    .json(&params::NewComment {
                        access_token: (*persisted).access_token.clone().unwrap(),
                        markdown: (*markdown).clone(),
                        post_id,
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<params::NewCommentResponse>()
                    .await
                    .unwrap();

                reset.emit(());
            });
        })
    };

    html! {
        <div class="col-12">
            <div class="form-group">
                <label for="new-comment">{"New comment"}</label>
                <textarea class="form-control" id="new-comment" rows="5" oninput={onmarkdown} disabled={disabled}></textarea>
            </div>
            <button type="button" class="btn btn-primary pull-right" onclick={onclick} disabled={disabled}>{"Submit"}</button>
        </div>
    }
}
