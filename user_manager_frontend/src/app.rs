use yew::{function_component, html, Html};

// From `yew_router`, we import:
// - `BrowserRouter` which handles routing in the browser,
// - `Routable` to turn an enum into valid routes,
// - `Switch` to render components based on the current route.
use yew_router::{BrowserRouter, Routable, Switch};

// We import our custom pages: `ArticlePage`, `HomePage`, and `LoginPage`.
// These are components that will be displayed when their routes are matched.
use crate::pages::{ArticlePage, HomePage, LoginPage};

// Define an enum `Route` that implements `Routable`. Each variant corresponds
// to a possible path (URL). The `#[at("/some/path")]` attributes indicate which
// URL paths lead to each variant.

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/article/:slug")]
    Article { slug: String },
    // If no other routes match, we return a NotFound variant.
    // The user will see the NotFound page if they navigate to a path that doesn't exist.
    #[not_found]
    #[at("/404")]
    NotFound,
}

// This function decides which component to render based on the `Route` enum value.
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Article { slug } => html! { <ArticlePage slug={slug} /> },
        Route::NotFound => html! { <h1>{ "You did something wrong!" }</h1> },
    }
}
// `App` is our root component. It sets up the `BrowserRouter` and the `Switch`
// so that the Yew Router system can determine which component to show.
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
           // The `Switch` component will call the `switch` function,
            // passing it the current route, and then render the component returned by `switch`.
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
