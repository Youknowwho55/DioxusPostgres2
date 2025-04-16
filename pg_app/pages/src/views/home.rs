use dioxus::prelude::*;
use components::db::{AddPost, Posts};






/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {

    rsx! {

        div { class: "flex flex-col justify-center pt-2 mt-2 items-center", AddPost {} }
        div { class: " flex flex-col justify-center items-center", Posts {} }
    }
}





