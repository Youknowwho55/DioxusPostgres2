use dioxus::prelude::*;
use views::{Blog, Home, Dashboard, Random, NotFound, Settings};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/dashboard")]
    Dashboard {},
    
    #[route("/settings")]
    Settings {},
    
    #[route("/random")]
    Random {},
    
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}