#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage as _};
use morum_base::{params, types};
use std::rc::Rc;
use web_sys::{HtmlInputElement, InputEvent};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

const API_PREFIX: &'static str = include_str!(concat!(env!("OUT_DIR"), "/api_prefix.txt"));

#[derive(Clone, Eq, PartialEq, Debug)]
enum PersistedAction {
    SetAccessToken(Option<String>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PersistedValue {
    pub access_token: Option<String>,
}

impl PersistedValue {
    fn new() -> Self {
        Self {
            access_token: LocalStorage::get("morum.access_token").ok(),
        }
    }
}

impl Reducible for PersistedValue {
    type Action = PersistedAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut persisted = self.as_ref().clone();
        match action {
            PersistedAction::SetAccessToken(new) => {
                persisted.access_token = new.clone();
                match new {
                    Some(new) => {
                        LocalStorage::set("morum.access_token", new)
                            .expect("set access token failed");
                    }
                    None => {
                        LocalStorage::delete("morum.access_token");
                    }
                }
            }
        }
        Rc::new(persisted)
    }
}

type Persisted = UseReducerHandle<PersistedValue>;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Categories /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Login => html! { <Login /> },
    }
}

#[function_component]
fn App() -> Html {
    let persisted = use_reducer(|| PersistedValue::new());

    html! {
        <BrowserRouter>
            <ContextProvider<Persisted> context={persisted.clone()}>
                <Nav />
                <div class="container m-3">
                    <Switch<Route> render={switch} />
                </div>
                <Footer />
            </ContextProvider<Persisted>>
        </BrowserRouter>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer class="text-center text-lg-start text-muted mt-3">
            <a href="https://that.world/legal.txt">{"Legal notice"}</a>{". "}
            {"Copyright (c) 2022 Wei Tang. morum is licensed under AGPL-3.0."}
        </footer>
    }
}

#[function_component]
fn Nav() -> Html {
    html! {
        <nav class="navbar navbar-light navbar-expand-sm">
            <span class="navbar-brand">
                <Link<Route> to={Route::Home}>{"morum"}</Link<Route>>
            </span>
            <ul class="navbar-nav"></ul>
            <NavLoginLink />
        </nav>
    }
}

#[function_component]
fn NavLoginLink() -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    if persisted.access_token.is_some() {
        let onlogout = Callback::from(move |_| {
            persisted.dispatch(PersistedAction::SetAccessToken(None));
            navigator.push(&Route::Home);
        });

        html! {
            <div class="login">
                <span class="navbar-text">
                    <a href="#" onclick={onlogout}>{"Log out"}</a>
                </span>
            </div>
        }
    } else {
        html! {
            <div class="login">
                <span class="navbar-text">
                    <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
                </span>
            </div>
        }
    }
}

#[function_component]
fn Login() -> Html {
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

#[function_component]
fn Categories() -> Html {
    let persisted = use_context::<Persisted>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let categories = use_state::<Option<Vec<types::Category>>, _>(|| None);

    if let Some(categories) = (*categories).clone() {
        html! {
            <>
                { categories.iter().map(|c| html! { <Category category={c.clone()} /> }).collect::<Html>() }
            </>
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
struct CategoryProps {
    pub category: types::Category
}

#[function_component]
fn Category(props: &CategoryProps) -> Html {
    html! {
        <>
            <div class="row mb-1">
                <h4>{props.category.title.clone()}<small>{props.category.topic.clone()}</small></h4>
            </div>
            <div class="row mb-3">
                { props.category.subcategories.iter()
                      .map(|s| html! { <Subcategory subcategory={s.clone()} /> }).collect::<Html>() }
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct SubcategoryProps {
    pub subcategory: types::Subcategory
}

#[function_component]
fn Subcategory(props: &SubcategoryProps) -> Html {
    html! {
        <div class="col-sm-6">
            <div class="card">
                <div class="card-body">
                    <h5 class="card-title"><a href="#">{props.subcategory.title.clone()}</a></h5>
                    <p class="card-text">{props.subcategory.topic.clone()}</p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
