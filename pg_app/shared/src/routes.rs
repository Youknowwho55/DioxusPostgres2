/// Solution 1: Trait-Based Routing (Recommended)

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
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

/// Trait for rendering route-aware components
#[async_trait(?Send)]  // Enable async if needed
pub trait PageRenderer {
    fn home() -> Element;
    fn blog( id: i32) -> Element;
    fn dashboard() -> Element;
    fn settings() -> Element;
    fn random() -> Element;
    fn not_found( segments: Vec<String>) -> Element;
}

/// Default router configuration
pub fn router_config() -> RouterConfig<Route> {
    RouterConfig::default()
        // Add any custom config here
        .history(MemoryHistory::default())
}