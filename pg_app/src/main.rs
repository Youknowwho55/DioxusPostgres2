#![allow(non_snake_case)]
use dioxus::prelude::*;
use views::{Blog, Home, Dashboard, Random, NotFound};
use assets::Assets;
use components::ui::toast::{ToastManager, ToastFrame};
use shared::Route;
use client::components::layout::AppLayout;
mod views;

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
        // Updated router syntax
        Router::<Route> {
            // Wrap all routes in `AppLayout`
            AppLayout {}
        }
        ToastFrame { manager: toast_manager }
    }
}




