#![allow(non_snake_case)]
use dioxus::prelude::*;
use views::{Blog, Home, Dashboard, Random, NotFound};
use views::layout::AppLayout;
use assets::Assets;
use components::ui::toast::{ToastManager, ToastFrame};
use routes::Route;  // Import from your shared crate

mod views;



fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let toast_manager = use_signal(|| ToastManager::default());
    use_context_provider(|| toast_manager);  // Changed from use_shared_state_provider

    rsx! {
        style { "{Assets::TAILWIND_CSS}" }
        document::Title { "My Desktop Application" }
        Router::<Route> {}
        ToastFrame { manager: toast_manager }
    }
}