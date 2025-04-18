// shared/src/routes.rs
use dioxus::prelude::*;
use super::page_components::*;

// Forward declare components (they'll be implemented in pages/)
#[component]
pub fn NavBar() -> Element;

#[component]
pub fn Home() -> Element;

#[component]
pub fn Blog() -> Element;

#[component]
pub fn BlogList() -> Element;

#[component]
pub fn BlogPost(name: String) -> Element;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/dashboard")]
        Dashboard {},
        #[route("/blog")]
        Blog {},
        #[route("/random")]
        Random {},
        
        #[nest("/blog")]
        #[layout(Blog)]
            #[route("/")]
            BlogList {},
            #[route("/:name")]
            BlogPost { name: String },
        #[end_layout]
        #[end_nest]
    #[end_layout]
    
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}