//pg_app/shared/src/routes.rs
// The Solution: Use String-Based Paths with Full Qualification

use dioxus::prelude::*;
use crate::home::Home;
use crate::dashboard::Dashboard;
use crate::blog::Blog;
use crate::random::Random;
use crate::not_found::NotFound;


/// Forward declarations for components
/// Doing this to try and avoid circular dependencies
// #[component]
// pub fn Home() -> Element {
//     rsx!{
//     }
// }

// #[component]
// pub fn Dashboard() -> Element {
//     rsx!{}
// }

// #[component]
// pub fn Blog() -> Element {
//     rsx!{}
// }

// #[component]
// pub fn Random() -> Element {
//     rsx!{}
// }

// #[component]
// pub fn BlogList() -> Element {
//     rsx!{}
// }

// #[component]
// pub fn BlogPost(name: String) -> Element {
//     rsx!{}
// }

// #[component]
// pub fn PageNotFound(route: Vec<String>) -> Element {
//     rsx!{}
// }

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]

pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    
    #[route("/dashboard")]
    Dashboard {},
    
    #[route("/blog")]
    Blog {},
    
    #[route("/random")]
    Random {},
    
//    #[end_layout]
    // #[route("/:..route")]
    // NotFound { route: Vec<String> },
}


use super::layout::footer::Footer;
use super::layout::navbar::Navbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: "flex-1 bg-gray-50",
                div { class: "container mx-auto px-4 py-6",
                    Outlet::<Route> {} // Renders matched child routes } }
                    Footer {}
                }
            }
        }
    }
}