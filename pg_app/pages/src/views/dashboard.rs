use dioxus::prelude::*;
use components::db::{AddPost, Posts};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize}; // Import enums if needed
use components::ui::Input;




/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Dashboard() -> Element {

    rsx! {
        div { class: "flex flex-col justify-center items-center", AddPost {} }
        div { class: " flex flex-col justify-center items-center", Posts {} }
    }
}





