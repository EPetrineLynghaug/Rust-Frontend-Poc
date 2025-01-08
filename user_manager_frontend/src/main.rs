use crate::components::App;
use dotenv::dotenv;
use yew::Renderer;

mod components;
mod user_manager;

fn main() {
    // Last inn miljøvariabler fra .env-filen
    dotenv().ok();
    Renderer::<App>::new().render();
}
