pub mod components;
mod pages;

use components::user_profile::{User, UserProfile};
use pages::{home::Home, not_found::NotFound, todo::Todo};
use yew::{function_component, html, ContextProvider, Html};
use yew_router::{components::Link, BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/todo")]
    Todo,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Todo => html! { <Todo /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ContextProvider<User> context={User {
                name: String::from("John"),
            }}>
                <UserProfile />
                <div>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::Todo}>{ "Todo" }</Link<Route>>
                </div>
                <Switch<Route> render={Switch::render(switch)} />
            </ContextProvider<User>>
        </BrowserRouter>
    }
}
