#![allow(non_snake_case)]

use dioxus::prelude::*;
use views::{Blog, Home, Navbar, Dashboard};

use assets::Assets;
use dioxus_desktop::{WindowBuilder, Config, LogicalSize};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(Navbar)]

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
        // Inlined CSS for reliability
        style { "{Assets::TAILWIND_CSS}" }
        document::Title { "My Desktop Application" }
        Router::<Route> {}
    }
}
