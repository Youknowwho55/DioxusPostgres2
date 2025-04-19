// //pg_app/shared/src/routes.rs
// // The Solution: Use String-Based Paths with Full Qualification




// use dioxus::prelude::*;

// /// Forward declarations for components
// /// Doing this to try and avoid circular dependencies
// #[component]
// pub fn Home() -> Element {
//     rsx!{}
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

// #[derive(Routable, Clone, Debug, PartialEq)]
// #[rustfmt::skip]

// pub enum Route {
//     #[layout(crate::pages::layout::app_layout::AppLayout)]
//     #[route("/")]
//     Home {},
    
//     #[route("/dashboard")]
//     Dashboard {},
    
//     #[route("/blog")]
//     Blog {},
    
//     #[route("/random")]
//     Random {},
    
//    #[end_layout]
//     #[route("/:..route")]
//     PageNotFound { route: Vec<String> },
// }


