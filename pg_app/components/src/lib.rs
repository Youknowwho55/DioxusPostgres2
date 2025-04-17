//pg_app/components/src/lib.rs
pub mod db;
pub mod ui;
pub mod layout;


use dioxus::prelude::*;
use dioxus_router::prelude::*;  // Make sure you're importing Route-specific stuff

use layout::AppLayout;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)] // Update layout imports
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
