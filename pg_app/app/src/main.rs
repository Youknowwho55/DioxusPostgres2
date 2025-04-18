#![allow(non_snake_case)]
use dioxus::prelude::*;
use components::ui::toast::{ToastManager, ToastFrame};
use assets::Assets;
use shared::Route;
use shared::routes::Route;
use pages::{blog::*, home::*};


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
        Router::<Route> {
            // Map route variants to page components
            Route::Home {} => Home {},
            Route::Blog {} => Blog {},
            Route::BlogList {} => BlogList {},

        ToastFrame { manager: toast_manager }
    }
}


