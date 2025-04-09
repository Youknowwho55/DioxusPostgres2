
use dioxus::prelude::*;

use views::{Blog, Home, Navbar};
use ui::{Assets, Navbar};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(Navbar)]

        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },
}


fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {

        // Inlined CSS for reliability
        style { "{Assets::MAIN_CSS}" }
        style { "{Assets::TAILWIND_CSS}" }


        Router::<Route> {}
    }
}
