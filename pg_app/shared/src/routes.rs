// pg_app/shared/src/routes.rs
use dioxus::prelude::*;
use views::layout::AppLayout;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
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
