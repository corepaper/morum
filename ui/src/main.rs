#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;
use yew::functional::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            <Nav />
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav class="navbar navbar-light navbar-expand-sm">
            <span class="navbar-brand">
                <a href="/">{"morum"}</a>
            </span>
            <ul class="navbar-nav"></ul>
            <NavLoginLink />
        </nav>
    }
}

#[function_component(NavLoginLink)]
fn nav_login_link() -> Html {
    html! {
        <div class="login">
            <span class="navbar-text">
                <a href="#">{"Login"}</a>{" -- "}<a href="#">{"Register"}</a>
            </span>
        </div>
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-md-10 offset-md-1">
                    <h3>
                        {"Log in to morum"}
                        <small>{" or "}<a href="/register">{"register"}</a></small>
                    </h3>
                </div>
            </div>
            <div class="row">
                <div class="col-md-6 offset-md-3">
                    <form>
                        <div class="form-group">
                            <label>{"Username"}</label>
                            <input class="form-control" type="text" />
                        </div>
                        <div class="form-group">
                            <label>{"Password"}</label>
                            <input class="form-control" type="password" />
                        </div>
                        <button class="btn btn-primary pull-right" type="submit">
                            {"Log in"}
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}

#[function_component(Register)]
fn register() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-md-10 offset-md-1">
                    <h3>
                        {"Register for morum"}
                        <small>{" or "}<a href="/login">{"log in"}</a></small>
                    </h3>
                </div>
            </div>
            <div class="row">
                <div class="col-md-6 offset-md-3">
                    <form>
                        <div class="form-group">
                            <label>{"Username"}</label>
                            <input class="form-control" type="text" />
                        </div>
                        <div class="form-group">
                            <label>{"Password"}</label>
                            <input class="form-control" type="password" />
                        </div>
                        <button class="btn btn-primary pull-right" type="submit">
                            {"Register"}
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
    }
}

fn main() {
    set_panic_hook();
    yew::start_app::<Main>();
}
