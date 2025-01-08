mod components;
mod user_manager;

use components::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}
