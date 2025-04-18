#![allow(non_snake_case)]
use dioxus::prelude::*;
use components::ui::toast::{ToastManager, ToastFrame};
use assets::Assets;
use shared::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let toast_manager = use_signal(|| ToastManager::default());
    use_context_provider(|| toast_manager);

    rsx! {
        style { "{Assets::TAILWIND_CSS}" }
        document::Title { "My Desktop Application" }
        Router::<Route> {}

        ToastFrame { manager: toast_manager }
    }
}