#![allow(non_snake_case)]
use dioxus::prelude::*;
use views::{Blog, Home, Dashboard}; // Removed Navbar from imports since it's used in layout
use views::layout::AppLayout;
use assets::Assets;

mod views;

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
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { "{Assets::TAILWIND_CSS}" }
        document::Title { "My Desktop Application" }
        Router::<Route> {}
    }
}