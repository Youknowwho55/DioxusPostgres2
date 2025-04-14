use dioxus::prelude::*;
use crate::Route;
use crate::views::navbar::Navbar;
use components::layout::Footer;

#[component]
pub fn AppLayout() -> Element {
    rsx!(
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: "flex-1 bg-gray-50",
                div { class: "container mx-auto px-4 py-6", Outlet::<Route> {} }
            }
            Footer {}
        }
    )
}