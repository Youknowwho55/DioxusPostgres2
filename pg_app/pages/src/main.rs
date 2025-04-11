
use dioxus::prelude::*;

use views::{Blog, Home, Navbar, Dashboard, TestHeader};
use assets::Assets;
use dioxus_desktop::{WindowBuilder, Config, LogicalSize};

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

        #[route("/dashboard")]
        Dashboard {},
        #[route("/test")]
        TestHeader {},
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
