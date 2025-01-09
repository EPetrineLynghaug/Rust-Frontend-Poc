use crate::components::App;

use yew::Renderer;

mod components;
mod user_manager;

fn main() {
    // Last inn miljøvariabler fra .env-filen

    Renderer::<App>::new().render();
}
