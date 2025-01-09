mod components;
mod user_manager;

use components::App;
use yew::Renderer;

/// Hovedfunksjonen som starter Yew-applikasjonen.
/// Rendrer `App`-komponenten definert i `components.rs`.
fn main() {
    // Hvis du ønsker logging i nettleserkonsollen, fjern kommentaren:
    // wasm_logger::init(wasm_logger::Config::default());

    Renderer::<App>::new().render();
}
