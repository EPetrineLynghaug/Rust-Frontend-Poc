// Bring the `App` component from our `app` module into scope.
use crate::app::App;

// Declare submodules in our project: `app`, `components`, `helpers`, and `pages`.
// These modules likely contain different parts of the application.
mod app;
mod components;
mod helpers;
mod pages;

// The main function is the entry point of our application.
fn main() {
    // `yew::Renderer::<App>::new()` creates a new renderer for our root component `App`.
    // `.render()` starts the Yew application, mounting it to the browser DOM.
    yew::Renderer::<App>::new().render();
}
