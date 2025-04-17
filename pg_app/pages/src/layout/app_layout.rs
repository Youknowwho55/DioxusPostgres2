// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared::routes::Route;
// use crate::footer::Footer;
// use crate::navbar::Navbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            // Navbar {}
            main { class: "flex-1 bg-gray-50",
                div { class: "container mx-auto px-4 py-6", Outlet::<Route> {} }
            }
                // Footer {}
        }
    }
}