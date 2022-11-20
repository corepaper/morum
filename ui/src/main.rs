#![recursion_limit = "1024"]

mod persisted;
mod login;
mod category_list;
mod post_list;
mod post;

pub use crate::persisted::*;

use console_error_panic_hook::set_once as set_panic_hook;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

const API_PREFIX: &'static str = include_str!(concat!(env!("OUT_DIR"), "/api_prefix.txt"));

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/category/:category_id")]
    Posts { category_id: String },
    #[at("/post/:post_id")]
    Post { post_id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <category_list::CategoryList /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Login => html! { <login::Login /> },
        Route::Posts { category_id } => html! { <post_list::PostList category_id={category_id} /> },
        Route::Post { post_id } => html! { <post::Post id={post_id} /> },
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
            <a href="https://that.world/legal.txt" target="_blank">{"Legal notice"}</a>{". "}
            {"Copyright (c) 2022 Wei Tang. morum is licensed under "}
            <a href="https://github.com/corepaper/morum" target="_blank">{"AGPL-3.0"}</a>
            {"."}
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

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
