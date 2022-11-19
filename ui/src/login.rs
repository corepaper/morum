use gloo_net::http::Request;
use morum_base::params;
use web_sys::{HtmlInputElement, InputEvent};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Persisted, PersistedAction, API_PREFIX};

#[function_component]
pub fn Login() -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    if persisted.access_token.is_some() {
        navigator.push(&Route::Home);
    }

    let username = use_state(|| "".to_owned());
    let password = use_state(|| "".to_owned());

    let onusername = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let onpassword = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |_| {
            let username = (*username).clone();
            let password = (*password).clone();
            let persisted = persisted.clone();
            let navigator = navigator.clone();

            yew::platform::spawn_local(async move {
                let res = Request::post(&(API_PREFIX.to_owned() + "/api/native/user/login"))
                    .json(&params::Login { username, password })
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<params::LoginResponse>()
                    .await
                    .unwrap();

                persisted.dispatch(PersistedAction::SetAccessToken(Some(res.access_token)));
                navigator.push(&Route::Home);
            });
        })
    };

    html! {
        <>
            <div class="row">
                <div class="col-md-10 offset-md-1">
                    <h3>
                        {"Log in to morum"}
                    </h3>
                </div>
            </div>
            <div class="row">
                <div class="col-md-6 offset-md-3">
                    <div class="form-group">
                        <label>{"Username"}</label>
                        <input class="form-control" type="text" value={(*username).clone()} oninput={onusername} />
                    </div>
                    <div class="form-group">
                        <label>{"Password"}</label>
                        <input class="form-control" type="password" value={(*password).clone()} oninput={onpassword} />
                    </div>
                    <button class="btn btn-primary pull-right" {onclick}>
                        {"Log in"}
                    </button>
                </div>
            </div>
        </>
    }
}
