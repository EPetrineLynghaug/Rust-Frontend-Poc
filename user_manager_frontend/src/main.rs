use crate::app::App;

mod app;
mod components;
mod helpers;
mod pages;

fn main() {
    yew::Renderer::<App>::new().render();
}
