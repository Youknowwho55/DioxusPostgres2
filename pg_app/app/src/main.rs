#![allow(non_snake_case)]
use dioxus::prelude::*;
use views::{Blog, Home, Dashboard, Random, NotFound};
use assets::Assets;
use components::ui::toast::{ToastManager, ToastFrame};
use client::components::layout::AppLayout;
// app/src/main.rs
use shared::{Route, router_config, PageRenderer};
use pages::AppPages;
fn main() {
    dioxus::launch(App);
}


use shared::PageRenderer;


#[component]
fn App() -> Element {
    let toast_manager = use_signal(|| ToastManager::default());
    use_context_provider(|| toast_manager);

    rsx! {

        style { "{Assets::TAILWIND_CSS}" }
        document::Title { "My Desktop Application" }
        // Updated router syntax
        // Router::<Route> {
        //     // Wrap all routes in `AppLayout`
        //     AppLayout {}
        // }
        Router {
            config: router_config(),
            Route::Home {} => AppPages::home,
            Route::Blog { id } => |cx| AppPages::blog(cx, *id),
            Route::Dashboard {} => AppPages::dashboard,
            Route::Settings {} => AppPages::settings,
            Route::Random {} => AppPages::random,
            Route::NotFound { segments } => |cx| AppPages::not_found(cx, segments.clone()),
        }
        ToastFrame { manager: toast_manager }
    }
}



