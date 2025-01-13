use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::pages::{ArticlePage, HomePage, LoginPage};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/article/:slug")]
    Article { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Article { slug } => html! { <ArticlePage slug={slug} /> },
        Route::NotFound => html! { <h1>{ "You did something wrong!" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
